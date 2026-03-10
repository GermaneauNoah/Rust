use crate::interfaces::lexicon::Lexicon;

pub const ENGLISH: Lexicon = Lexicon {
    blank: "blank",
    candidate: "candidate",
    voter: "voter",
    invalid: "invalid",
    has_already_voted: "has already voted",
    accepted_vote: "accepted vote",
    invalid_vote: "invalid vote",
    vote_enregistered: "vote registered for",
    blank_vote_registered: "blank vote registered for",
    invalid_vote_registered: "invalid vote registered for",
    error: "error",
    score: "score",
    unknown_command: "unknown command",
    has_voted : "has voted",
    command : "Enter a command (vote <name> <candidate>, voters, scores, quit):",
    voter_required: "Voter name required",
};
