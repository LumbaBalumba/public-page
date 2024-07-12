use rocket::{
    fs::{FileServer, NamedFile},
    get, launch, routes,
};

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open("templates/index.html").await.ok()
}

#[get("/favicon.ico")]
async fn favicon() -> Option<NamedFile> {
    NamedFile::open("static/img/favicon.ico").await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, favicon])
        .mount("/static", FileServer::from("static"))
}
