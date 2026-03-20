use axum::{extract::{State, Form}, response::IntoResponse};
use crate::storage::Strorage; 
use super::{AxumState, AxumError, html_formatter};
use crate::interfaces::show_vote_outcome;
use crate::use_case::VoteForm;

pub async fn get_index<Store: Strorage>(
    State(app_state): State<AxumState<Store>>,
) -> Result<impl IntoResponse, AxumError> {
    let machine = app_state.controller.get_voting_machine().await.map_err(anyhow::Error::msg)?;
    Ok(html_formatter::index(&app_state.routes, &app_state.lexicon, &machine))
}

pub async fn get_results<Store: Strorage>(
    State(app_state): State<AxumState<Store>>,
) -> Result<impl IntoResponse, AxumError> {
    let machine = app_state.controller.get_voting_machine().await.map_err(anyhow::Error::msg)?;
    Ok(html_formatter::voting_machine(&app_state.routes, &app_state.lexicon, &machine))
}

pub async fn vote<Store: Strorage>(
    State(app_state): State<AxumState<Store>>,
    Form(vote_form): Form<VoteForm>,
) -> Result<impl IntoResponse, AxumError> {
    let outcome = app_state.controller.vote(vote_form).await?;
    Ok(show_vote_outcome(outcome, &app_state.lexicon))
}