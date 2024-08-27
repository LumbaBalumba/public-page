use rocket::launch;

pub mod app;

#[cfg(test)]
mod tests;

#[launch]
fn rocket() -> _ {
    app::rocket()
}
