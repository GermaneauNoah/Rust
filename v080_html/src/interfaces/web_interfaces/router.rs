use axum::{routing::{get, post}, Router};
use crate::storage::Strorage;
use super::{AxumState, web_routes::WebRoutes, html_handlers};

pub fn make_router<Store: Strorage + Clone + Send + Sync + 'static>(
    app_state: AxumState<Store>,
    routes: &WebRoutes,
) -> Router {
    Router::new()
        .route(routes.index, get(html_handlers::get_index))
        .route(routes.vote, post(html_handlers::vote))
        .route(routes.results, get(html_handlers::get_results))
        .with_state(app_state)
}