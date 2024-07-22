use axum::{Router, routing::get};
use crate::handlers;
use crate::constants::route_paths;

pub fn routes() -> Router {
    Router::new().route(route_paths::ROOT, get(handlers::health_check::health_check))
}
