use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap as Map, BTreeSet as Set};
use crate::domain::{VoteOutcome, Votingmachine};
use crate::use_case::VoteForm;

#[derive(Deserialize)]
pub struct VoteFormV1 {
  pub voter: String,
  pub candidate: String,
}

impl From<VoteFormV1> for VoteForm {
    fn from(form_v1: VoteFormV1) -> Self {
        Self {
            voter: form_v1.voter,
            candidat: form_v1.candidate, // Remarque : 'candidat' côté VoteForm (domaine) et 'candidate' côté Web
        }
    }
}

#[derive(Serialize)]
pub enum VoteOutcomeV1 {
  AcceptedVote(String, String),
  HasAlreadyVoted(String),
  BlankVote(String),
  InvalidVote(String),
}

impl From<VoteOutcome> for VoteOutcomeV1 {
    fn from(outcome: VoteOutcome) -> Self {
        match outcome {
            VoteOutcome::AcceptedVote(voter, candidate) => Self::AcceptedVote(voter.0, candidate.0),
            VoteOutcome::HasAlreadyVoted(voter) => Self::HasAlreadyVoted(voter.0),
            VoteOutcome::BlankVote(voter) => Self::BlankVote(voter.0),
            VoteOutcome::InvalidVote(voter) => Self::InvalidVote(voter.0),
        }
    }
}

#[derive(Serialize)]
pub struct VotingMachineV1 {
  voters: Set<String>,
  scores: Map<String, usize>,
  blank_score: usize,
  invalid_score: usize,
}

impl From<Votingmachine> for VotingMachineV1 {
    fn from(machine: Votingmachine) -> Self {
        let voters = machine.get_voters_who_voted().into_iter().map(|v| v.0.clone()).collect();
        
        let scoreboard = machine.get_scoreboard();
        let scores = scoreboard.scores.iter().map(|(c, s)| (c.0.clone(), s.0)).collect();
        
        Self {
            voters,
            scores,
            blank_score: scoreboard.blank_score.0,
            invalid_score: scoreboard.invalid_score.0,
        }
    }
}
