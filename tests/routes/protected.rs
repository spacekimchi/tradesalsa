use crate::helpers::{spawn_app, assert_is_redirect_to};

#[tokio::test]
async fn get_protected() {
    let app = spawn_app().await;

    // It will first reject
    let response = app.get_protected().await;
    assert_eq!(response.status(), reqwest::StatusCode::INTERNAL_SERVER_ERROR);

    // TODO: This is the same as the post_login test
    let body = serde_json::json!({
        "email": app.test_user.email,
        "password": app.test_user.password,
    });

    let response = app.post_login(&body).await;
    assert_eq!(response.status(), reqwest::StatusCode::SEE_OTHER);
    assert_is_redirect_to(&response, "/");

    let response = app.get_protected().await;
    assert_eq!(response.status(), reqwest::StatusCode::OK);
}
