use rocket::{
    fs::{FileServer, NamedFile},
    get, launch, routes,
};
use rocket_dyn_templates::{Template, context};

#[get("/")]
async fn index() -> Template {
    Template::render("index", context! {})
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
        .attach(Template::fairing())
}
