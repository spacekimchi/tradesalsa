use axum::Extension;
use axum::response::{Html, IntoResponse};
use crate::startup::AppState;
use crate::template_helpers::{render_content, RenderTemplateParams};
use crate::constants::html_templates;

pub async fn homepage(Extension(state): Extension<AppState>) -> impl IntoResponse {
    let mut context = tera::Context::new();
    let boo = "asdf";
    context.insert("boo", &boo);

    match render_content(
        &RenderTemplateParams::new(html_templates::HOMEPAGE, &state.tera)
        .with_context(&context)
    ) {
        Ok(homepage_template) => Html(homepage_template).into_response(),
        Err(e) => e.into_response()
    }
}

