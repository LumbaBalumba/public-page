use crate::app;
use rocket::{
    self,
    http::{ContentType, Status},
    local::asynchronous::Client,
};

#[rocket::async_test]
async fn index_test() {
    let client = Client::tracked(app::rocket())
        .await
        .expect("valid rocket instance");
    let response = client.get("/").dispatch().await;

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::HTML))
}
