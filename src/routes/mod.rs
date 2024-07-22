use axum::Router;
use crate::constants::route_paths;

mod health_check;
mod homepage;
mod auth;
mod protected;

pub fn homepage_routes() -> Router {
    Router::new().nest(route_paths::ROOT, homepage::routes())
}

pub fn auth_routes() -> Router {
    Router::new().nest(route_paths::ROOT, auth::routes())
}

pub fn health_check_routes() -> Router {
    Router::new().nest(route_paths::HEALTH, health_check::routes())
}

pub fn protected_routes() -> Router {
    Router::new().nest(route_paths::PROTECTED, protected::routes())
}
