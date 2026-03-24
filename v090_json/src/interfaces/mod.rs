pub mod cli_interface;
pub mod lexicon;
pub mod lexicons;
pub mod web_interfaces;

use crate::domain::VoteOutcome;
use crate::interfaces::lexicon::Lexicon;

pub fn show_vote_outcome(outcome: VoteOutcome, lexicon: &Lexicon) -> String {
    match outcome {
        VoteOutcome::BlankVote(v) => format!("{} {}.", lexicon.blank_vote_registered, v.0),
        VoteOutcome::InvalidVote(v) => format!("{} {}.", lexicon.invalid_vote_registered, v.0),
        VoteOutcome::AcceptedVote(v, c) => format!("{} {} {}.", v.0, lexicon.has_voted, c.0),
        VoteOutcome::HasAlreadyVoted(v) => format!("{} : {} {}", lexicon.error, v.0, lexicon.has_already_voted),
    }
}
