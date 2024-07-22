use crate::helpers::spawn_app;

#[tokio::test]
async fn homepage() {
    let app = spawn_app().await;

    let response = app.get_homepage_html().await;
    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let html_page = response.text().await.expect("Failed to read the response body");
    assert!(html_page.contains("HOMEPAGE BABY!"));
}
