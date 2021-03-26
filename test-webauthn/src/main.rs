extern crate actix_cors;
extern crate actix_files;
extern crate actix_session;
extern crate actix_web;
extern crate async_std;
extern crate base64;
extern crate env_logger;
extern crate lru;
extern crate openssl;
extern crate rustls;
extern crate serde;
extern crate webauthn_rs;

use actix_cors::Cors;
use actix_files::Files;
use actix_session::CookieSession;
use actix_web::{
    error, http, middleware::Logger, web, App, Error, HttpRequest, HttpResponse, HttpServer,
    Responder,
};
use env_logger::Env;
use serde::{Deserialize, Serialize};
use std::{
    borrow::BorrowMut,
    sync::{Arc, Mutex},
};
use webauthn_rs::ephemeral::WebauthnEphemeralConfig;
use webauthn_rs::proto::{PublicKeyCredential, RegisterPublicKeyCredential};

mod actors;
mod crypto;

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
        Ok(()) => HttpResponse::Ok().json("body"),
        Err(e) => HttpResponse::InternalServerError().body(format!("{}", e)),
    };
    Ok(res)
}

async fn challenge_login(
    state: web::Data<WebauthnActor>,
    val: web::Path<Login>,
) -> Result<HttpResponse, Error> {
    dbg!("Hello from challenge");
    let actor_res = state.challenge_authenticate(&val.username).await;
    let res = match actor_res {
        Ok(chal) => HttpResponse::Ok().json(&chal),
        Err(e) => HttpResponse::InternalServerError().body(format!("{}", e)),
    };
    Ok(res)
}

async fn login(
    state: web::Data<WebauthnActor>,
    val: web::Path<Login>,
    body: web::Json<PublicKeyCredential>,
) -> Result<HttpResponse, Error> {
    let res = match state.authenticate(&val.username, &body).await {
        Ok(()) => HttpResponse::Ok().json("Success"),
        Err(err) => HttpResponse::NotFound().body(format!("{}", err)),
    };
    Ok(res)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let cmd_opt = CmdOptions::new(
        String::from("/"),
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
                    .domain(domain.as_str())
                    .path(prefix.as_str())
                    .name("webauthnrs")
                    .secure(true),
            )
            .route(
                "/auth/challenge/register/{username}",
                web::post().to(challenge_register),
            )
            .route(
                "/auth/challenge/login/{username}",
                web::post().to(challenge_login),
            )
            .route("/auth/register/{username}", web::post().to(register))
            .route("/auth/login/{username}", web::post().to(login))
            .service(Files::new("/build", "public/build").show_files_listing())
            .service(Files::new("/", "./public/").index_file("index.html"))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
