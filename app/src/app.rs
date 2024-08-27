use rocket::{
    fs::{FileServer, NamedFile},
    get, routes, Build, Rocket,
};
use rocket_dyn_templates::{context, Template};

#[get("/")]
async fn index() -> Template {
    Template::render("index", context! {})
}

#[get("/favicon.ico")]
async fn favicon() -> Option<NamedFile> {
    NamedFile::open("static/img/favicon.ico").await.ok()
}

pub fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![index, favicon])
        .mount("/static", FileServer::from("static"))
        .attach(Template::fairing())
}
