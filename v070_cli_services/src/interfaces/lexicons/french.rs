use crate::interfaces::lexicon::Lexicon;

pub const FRENCH: Lexicon = Lexicon {
    blank: "blanc",
    candidate: "candidat",
    voter: "votant",
    invalid: "nul",
    has_already_voted: "a déjà voté",
    accepted_vote: "vote accepté",
    invalid_vote: "vote nul",
    vote_enregistered: "Vote enregistré pour",
    blank_vote_registered: "Vote blanc enregistré pour",
    invalid_vote_registered: "Vote nul enregistré pour",
    error: "erreur",
    score: "score",
    unknown_command: "commande inconnue",
    has_voted : "a voté",
    command: "Saisis une commande (voter <nom> <candidat>, votants, scores, quitter):",
    voter_required: "Nom du votant requis",
};
