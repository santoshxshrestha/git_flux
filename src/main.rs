#![allow(unused)]
use actix_files::Files;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct HelloTemplate {
    name: String,
    language: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    let template = HelloTemplate {
        name: "Santosh".to_string(),
        language: "rust".to_string(),
    };
    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(Files::new("/static", "./gitflux"))
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
