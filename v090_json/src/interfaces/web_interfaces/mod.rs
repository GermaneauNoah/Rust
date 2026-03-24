pub mod html_formatter;
pub mod html_handlers;
pub mod router;
pub mod web_routes;
pub mod json;

use axum::{response::{IntoResponse, Response}, http::StatusCode};
use thiserror::Error;
use crate::use_case::VotingController;
use crate::interfaces::lexicon::Lexicon;
use self::web_routes::WebRoutes;

#[derive(Error, Debug)]
#[error("Error: {0}")]
pub struct AxumError(#[from] anyhow::Error);

impl IntoResponse for AxumError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self),
        )
            .into_response()
    }
}

#[derive(Clone)]
pub struct AxumState<Store> {
    pub controller: VotingController<Store>,
    pub routes: WebRoutes,
    pub lexicon: Lexicon,
}
