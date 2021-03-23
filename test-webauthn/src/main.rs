extern crate actix_session;
extern crate actix_web;
extern crate async_std;
extern crate base64;
extern crate lru;
extern crate openssl;
extern crate rustls;
extern crate serde;
extern crate webauthn_rs;

use actix_session::CookieSession;
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use proto::PublicKeyCredential;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use webauthn_rs::ephemeral::WebauthnEphemeralConfig;

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

#[derive(Debug)]
struct AppState {
    counter: i32,
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

async fn login(
    val: web::Json<Login>,
   
) -> Result<HttpResponse, std::io::Error> {
   
    let output = HttpResponse::Ok().json(MyObj::new(&val.username, 0));
    Ok(output)
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
        App::new()
            .wrap(
                CookieSession::signed(&[0; 32])
                    .domain(prefix.as_str())
                    .name("webauthnrs")
                    .secure(true),
            )
            .app_data(app_state.clone())
            .route("/", web::post().to(login))
            .route("/spider", web::post().to(login))
    })
    .bind("127.0.0.1:5000")?
    .run()
    .await
}
