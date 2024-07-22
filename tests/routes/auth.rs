use crate::helpers::{
    spawn_app,
    assert_is_redirect_to,
    fake_email,
    rand_digit,
    rand_lowercase,
    rand_uppercase,
    rand_special_char,
};

#[tokio::test]
async fn get_login() {
    let app = spawn_app().await;

    let response = app.get_login(None).await;
    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let html_page = response.text().await.expect("Failed to read the response body");
    assert!(html_page.contains(r#"<input name="email" id="email""#));
    assert!(html_page.contains(r#"<input name="password" id="password" type="password""#));
    assert!(html_page.contains(r#"<input type="submit" value="login""#));
    assert!(!html_page.contains(r#"<input type="hidden" name="next""#));
}

#[tokio::test]
async fn get_login_with_next() {
    let app = spawn_app().await;

    let query_params = [("next", "/protected")];
    let response = app.get_login(Some(&query_params)).await;
    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let html_page = response.text().await.expect("Failed to read the response body");
    assert!(html_page.contains(r#"<input name="email" id="email""#));
    assert!(html_page.contains(r#"<input name="password" id="password" type="password""#));
    assert!(html_page.contains(r#"<input type="submit" value="login""#));
    assert!(html_page.contains(r#"<input type="hidden" name="next" value="&#x2F;protected""#));
}

#[tokio::test]
async fn post_login() {
    let app = spawn_app().await;
    let body = serde_json::json!({
        "email": app.test_user.email,
        "password": app.test_user.password,
    });

    let response = app.post_login(&body).await;
    assert_eq!(response.status(), reqwest::StatusCode::SEE_OTHER);
    assert_is_redirect_to(&response, "/");
}

#[tokio::test]
async fn post_login_with_next() {
    let app = spawn_app().await;
    let next_route = "/protected";
    let body = serde_json::json!({
        "email": app.test_user.email,
        "password": app.test_user.password,
        "next": next_route
    });

    let response = app.post_login(&body).await;
    assert_eq!(response.status(), reqwest::StatusCode::SEE_OTHER);
    assert_is_redirect_to(&response, next_route);
}

#[tokio::test]
async fn get_register() {
    let app = spawn_app().await;

    let response = app.get_register().await;
    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let html_page = response.text().await.expect("Failed to read the response body");
    assert!(html_page.contains(r#"<input name="email" id="email""#));
    assert!(html_page.contains(r#"<input name="password" id="password" type="password""#));
    assert!(html_page.contains(r#"<input type="submit" value="Register""#));
}

#[tokio::test]
async fn post_register() {
    let app = spawn_app().await;
    let mut password = String::from(rand_digit());
    let body = serde_json::json!({
        "email": fake_email(),
        "password": password
    });

    let response = app.post_register(&body).await;
    assert_eq!(response.status(), reqwest::StatusCode::SEE_OTHER);
    assert_is_redirect_to(&response, "/register");

    password.push(rand_lowercase());
    let body = serde_json::json!({
        "email": fake_email(),
        "password": password
    });

    let response = app.post_register(&body).await;
    assert_eq!(response.status(), reqwest::StatusCode::SEE_OTHER);
    assert_is_redirect_to(&response, "/register");

    password.push(rand_uppercase());
    let body = serde_json::json!({
        "email": fake_email(),
        "password": password
    });

    let response = app.post_register(&body).await;
    assert_eq!(response.status(), reqwest::StatusCode::SEE_OTHER);
    assert_is_redirect_to(&response, "/register");

    password.push(rand_special_char());
    let body = serde_json::json!({
        "email": fake_email(),
        "password": password
    });

    let response = app.post_register(&body).await;
    assert_eq!(response.status(), reqwest::StatusCode::SEE_OTHER);
    assert_is_redirect_to(&response, "/register");

    for _ in 5..12 {
        password.push(rand_lowercase());
    }
    let body = serde_json::json!({
        "email": fake_email(),
        "password": password
    });


    let response = app.post_register(&body).await;
    assert_eq!(response.status(), reqwest::StatusCode::SEE_OTHER);
    assert_is_redirect_to(&response, "/");
}

