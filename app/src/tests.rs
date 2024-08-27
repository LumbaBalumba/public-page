use crate::app;
use rocket::{self, http::Status, local::asynchronous::Client};

#[rocket::async_test]
async fn index_test() {
    let client = Client::tracked(app::rocket())
        .await
        .expect("valid rocket instance");
    let response = client.get("/").dispatch().await;

    assert_eq!(response.status(), Status::Ok);
}
