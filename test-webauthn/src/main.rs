extern crate actix_cors;
extern crate actix_session;
extern crate actix_web;
extern crate async_std;
extern crate base64;
extern crate lru;
extern crate openssl;
extern crate rustls;
extern crate serde;
extern crate webauthn_rs;

use actix_cors::Cors;
use actix_session::CookieSession;
use actix_web::{error, http, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use proto::PublicKeyCredential;
use serde::{Deserialize, Serialize};
use std::{
    borrow::BorrowMut,
    sync::{Arc, Mutex},
};
use webauthn_rs::ephemeral::WebauthnEphemeralConfig;
use webauthn_rs::proto::RegisterPublicKeyCredential;

mod actors;
mod base64_data;
mod crypto;
mod proto;

use actors::WebauthnActor;
#[derive(Serialize, Deserialize)]
struct MyObj {
    name: String,
    counter: i32,
}
impl MyObj {
    pub fn new(name: &str, counter: i32) -> Self {
        MyObj {
            name: name.to_string(),
            counter,
        }
    }
}

#[derive(Deserialize, Debug)]
struct Login {
    username: String,
}

struct CmdOptions {
    prefix: String,
    rp_name: String,
    rp_origin: String,
    rp_id: String,
    bind: String,
}
impl CmdOptions {
    pub fn new(
        prefix: String,
        rp_name: String,
        rp_origin: String,
        rp_id: String,
        bind: String,
    ) -> Self {
        Self {
            prefix,
            rp_name,
            rp_origin,
            rp_id,
            bind,
        }
    }
}

async fn challenge_register(
    state: web::Data<WebauthnActor>,
    val: web::Path<Login>,
) -> Result<HttpResponse, Error> {
    let actor_res = state.challenge_register(val.username.clone()).await;

    let res = match actor_res {
        Ok(chal) => HttpResponse::Ok().json(&chal),
        Err(e) => HttpResponse::InternalServerError().body(format!("{}", e)),
    };
    Ok(res)
}

async fn register(
    state: web::Data<WebauthnActor>,
    val: web::Path<Login>,
    body: web::Json<RegisterPublicKeyCredential>,
) -> Result<HttpResponse, Error> {
    let actor_res = state.register(&val.username, &body).await;
    let res = match actor_res {
        Ok(()) => HttpResponse::Ok().body("body"),
        Err(e) => HttpResponse::InternalServerError().body(format!("{}", e)),
    };
    Ok(res)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cmd_opt = CmdOptions::new(
        String::from("/auth"),
        String::from("localhost"),
        String::from("http://localhost:8080"),
        String::from("localhost"),
        String::from("localhost:8080"),
    );
    let prefix = cmd_opt.prefix.clone();
    let domain = cmd_opt.rp_id.clone();
    let wan_c = WebauthnEphemeralConfig::new(
        cmd_opt.rp_name.as_str(),
        cmd_opt.rp_origin.as_str(),
        cmd_opt.rp_id.as_str(),
        None,
    );
    let wan = WebauthnActor::new(wan_c);
    let app_state = web::Data::new(wan);
    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .app_data(app_state.clone())
            .wrap(
                CookieSession::signed(&[0; 32])
                    .domain(prefix.as_str())
                    .name("webauthnrs")
                    .secure(true),
            )
            .wrap(cors)
            .route(
                "/auth/challenge/register/{username}",
                web::post().to(challenge_register),
            )
            .route("/auth/register/{username}", web::post().to(register))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
