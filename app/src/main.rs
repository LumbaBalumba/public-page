use rocket::launch;

pub mod app;
mod tests;

#[launch]
fn rocket() -> _ {
    app::rocket()
}
