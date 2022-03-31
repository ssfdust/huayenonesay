extern crate database;
use actix_session::{storage::CookieSessionStore, Session, SessionMiddleware};
use actix_web::cookie::Key;
use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpServer, Responder};
use chrono::prelude::*;
use database::{get_img_by_day, get_random_saying};
use env_logger;
use serde_json::json;
use std::env;
use serde::Deserialize;

#[derive(Deserialize)]
struct BgArgs {
    device: String,
}

#[derive(Default, Debug)]
struct Cached {
    onesay: String,
    chapter: String,
    date: String
}

#[get("/api/huayen/onesay")]
async fn onesay(session: Session) -> impl Responder {
    let onesay: String;
    let chapter: String;
    let local: DateTime<Local> = Local::now();
    let datenowstr: String = local.format("%Y%m%d").to_string();
    let mut cached = Cached::default();
    for (key, val) in session.entries().iter() {
        if key.eq("onesay") {
            cached.onesay = val.replace("\"", "").to_owned()
        }
        if key.eq("date") {
            cached.date = val.replace("\"", "").to_owned()
        }
        if key.eq("chapter") {
            cached.chapter = val.replace("\"", "").to_owned()
        }
    }
    match cached {
        cached if cached.date.eq(&datenowstr) && cached.onesay.len() > 0 && cached.chapter.len() > 0 => {
            onesay = cached.onesay.to_owned();
            chapter = cached.chapter.to_owned();
        }
        _ => {
            (onesay, chapter) = get_random_saying();
            session.insert("date", &datenowstr).unwrap();
            session.insert("chapter", &chapter).unwrap();
            session.insert("onesay", &onesay).unwrap();
        }
    }
    web::Json(
        json!({ "code": 0, "msg": "get one say successfully.", "data": { "onesay": onesay, "chapter": chapter } }),
    )
}

#[get("/api/img/bg")]
async fn get_img(bg_args: web::Query<BgArgs>, _: Session) -> impl Responder {
    let local: DateTime<Local> = Local::now();
    web::Json(
        json!({ "code": 0, "msg": "get background image successfully.", "data": {"url": get_img_by_day(local.day(), &bg_args.device)}}),
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let bindaddr = env::var("BINDADDR").unwrap_or("127.0.0.1".to_owned());
    let key = env::var("SESSION_KEY").unwrap_or("secret_key".to_owned());
    let port = env::var("BINDPORT").unwrap_or("5001".to_owned());
    let port = port.parse::<u16>().unwrap();
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    println!("Server runs on {}:{}", bindaddr, port);
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(key.as_ref()))
                    .cookie_secure(false)
                    .build(),
            )
            .service(onesay)
            .service(get_img)
    })
    .bind((bindaddr.to_string(), port))?
    .run()
    .await
}
