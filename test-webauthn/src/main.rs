extern crate actix_web;
extern crate base64;
extern crate serde;

use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use proto::PublicKeyCredential;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

mod base64_data;
mod proto;

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
    counter: Mutex<i32>,
}

async fn login(
    val: web::Json<Login>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, std::io::Error> {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    *counter += 1;
    let output = HttpResponse::Ok().json(MyObj::new(&val.username, *counter));
    Ok(output)
}
async fn index(data: web::Data<AppState>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    format!("Request number: {}", counter) // <- response with count
}
async fn register(
    info: web::Json<Login>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, std::io::Error> {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    dbg!(&info.username);
    let output = HttpResponse::Ok().json(MyObj::new(&info.username, *counter));
    Ok(output)
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppState {
        counter: Mutex::new(0),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone())
            .route("/", web::get().to(login))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
