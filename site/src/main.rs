use std::{fs, io::Result};

use actix_files::Files;
use actix_web::{get, http::header::ContentType, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    let body = fs::read_to_string("templates/index.html").unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body)
}

#[get("/sites")]
async fn sites() -> impl Responder {
    let body = fs::read_to_string("templates/sites.html").unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body)
}

#[get("/favicon.ico")]
async fn favicon() -> actix_web::Result<actix_files::NamedFile> {
    Ok(actix_files::NamedFile::open("static/img/favicon.ico")?)
}

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(Files::new("/static", "static/").show_files_listing())
            .service(index)
            .service(sites)
            .service(favicon)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}