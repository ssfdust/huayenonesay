extern crate database;
use actix_web::{get, web, App, HttpServer, Responder};
use database::get_random_saying;
use serde_json::json;

#[get("/api/huayen/onesay")]
async fn onesay() -> impl Responder {
    web::Json(
        json!({ "code": 0, "msg": "get one say successfully.", "data": { "onesay": get_random_saying() } }),
    )
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(onesay))
        .bind(("127.0.0.1", 18080))?
        .run()
        .await
}
