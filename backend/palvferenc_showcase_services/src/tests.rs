use actix_web::{http};

#[actix_rt::test]
async fn test_get_users_ok() {
    let resp = crate::services::get_users().await;
    assert_eq!(resp.status(), http::StatusCode::OK);

    let response_body = match resp.body().as_ref() {
        Some(actix_web::body::Body::Bytes(bytes)) => bytes,
        _ => panic!("Response error"),
    };

    assert_eq!(response_body, "[{\"id\":1,\"name\":\"Test user 1\"},{\"id\":2,\"name\":\"Test user 2\"},{\"id\":3,\"name\":\"Test user 3\"},{\"id\":4,\"name\":\"Test user 4\"}]");
}
