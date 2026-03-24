use axum::{extract::{State, Json}, response::IntoResponse};
use crate::storage::Strorage;
use crate::interfaces::web_interfaces::{AxumState, AxumError};
use super::v1_formatter::{VoteFormV1, VoteOutcomeV1, VotingMachineV1};
use crate::use_case::VoteForm;

pub async fn vote<Store: Strorage>(
    State(app_state): State<AxumState<Store>>,
    Json(vote_form): Json<VoteFormV1>,
) -> Result<impl IntoResponse, AxumError> {
    let form: VoteForm = vote_form.into();
    let outcome = app_state.controller.vote(form).await?;
    let outcome_v1: VoteOutcomeV1 = outcome.into();
    Ok(Json(outcome_v1))
}

pub async fn get_results<Store: Strorage>(
    State(app_state): State<AxumState<Store>>,
) -> Result<impl IntoResponse, AxumError> {
    let machine = app_state.controller.get_voting_machine().await?;
    let machine_v1: VotingMachineV1 = machine.into();
    Ok(Json(machine_v1))
}
