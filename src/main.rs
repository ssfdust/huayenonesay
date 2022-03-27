extern crate database;
use actix_web::{get, web, App, HttpServer, Responder};
use database::get_random_saying;
use serde_json::json;
use std::env;

#[get("/api/huayen/onesay")]
async fn onesay() -> impl Responder {
    web::Json(
        json!({ "code": 0, "msg": "get one say successfully.", "data": { "onesay": get_random_saying() } }),
    )
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let bindaddr = env::var("BINDADDR").unwrap_or("127.0.0.1".to_owned());
    let port = env::var("BINDPORT").unwrap_or("18080".to_owned());
    let port = port.parse::<u16>().unwrap();
    println!("Server runs on {}:{}", bindaddr, port);
    HttpServer::new(|| App::new().service(onesay))
        .bind((bindaddr.to_string(), port))?
        .run()
        .await
}
