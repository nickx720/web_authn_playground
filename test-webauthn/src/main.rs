extern crate actix_web;
extern crate serde;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MyObj {
    name: String,
}
impl MyObj {
    pub fn new(name: &str) -> Self {
        MyObj {
            name: name.to_string(),
        }
    }
}

async fn login() -> Result<HttpResponse, std::io::Error> {
    let output = HttpResponse::Ok().json(MyObj::new("Nick"));
    Ok(output)
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(login)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
