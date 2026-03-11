#[derive(PartialEq, Eq, Clone)]
pub struct Lexicon {
    pub blank: &'static str,
    pub candidate: &'static str,
    pub voter: &'static str,
    pub invalid: &'static str,
    pub has_already_voted: &'static str,
    pub accepted_vote: &'static str,
    pub invalid_vote: &'static str,
    pub vote_enregistered: &'static str,
    pub blank_vote_registered: &'static str,
    pub invalid_vote_registered: &'static str,
    pub error : &'static str,
    pub score: &'static str,
    pub unknown_command: &'static str,
    pub has_voted : &'static str,
    pub command: &'static str,
    pub voter_required: &'static str,
}

