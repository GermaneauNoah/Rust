use crate::domain::{AttendencesSheet, Scoreboard, VoteOutcome};
use crate::interfaces::lexicon::{Lexicon};
use crate::storage::Strorage;
use crate::use_case::{VotingController, VoteForm};

pub fn show_vote_outcome(outcome: VoteOutcome, lexicon: &Lexicon) -> String {
    match outcome {
        VoteOutcome::BlankVote(v) => format!("{} {}.", lexicon.blank_vote_registered, v.0),
        VoteOutcome::InvalidVote(v) => format!("{} {}.", lexicon.invalid_vote_registered, v.0),
        VoteOutcome::AcceptedVote(v, c) => format!("{} {} {}.", c.0,lexicon.has_voted, v.0),
        VoteOutcome::HasAlreadyVoted(v) => format!("{} : {} {}",lexicon.error, v.0, lexicon.has_already_voted),
    }
}

pub fn show_scoreboard(scoreboard: &Scoreboard, lexicon: &Lexicon) -> String {
    let mut output = String::new();
    output.push_str("\n-------------------\n");
    output.push_str(&format!("{}:\n", lexicon.score));
    output.push_str(&format!("{} : {}\n", lexicon.blank, scoreboard.blank_score.0));
    output.push_str(&format!("{} : {}\n", lexicon.invalid, scoreboard.invalid_score.0));
    for (candidat, score) in &scoreboard.scores {
        output.push_str(&format!("{}: {}\n", candidat.0, score.0));
    }
    output.push_str("-------------------\n");
    output
}

pub fn show_attendance_sheet(sheet: &AttendencesSheet, lexicon: &Lexicon) -> String {
    let mut output = String::new();
    output.push_str("\n-------------------\n");
    output.push_str(&format!("{}:\n", lexicon.voter));
    for voter in sheet.voters() {
        output.push_str(&format!("- {}\n", voter.0));
    }
    output.push_str("-------------------\n");
    output
}

/// Traite une ligne de commande et retourne la réponse à afficher.
///
/// Format attendu selon la commande :
///   - "votants"
///   - "scores"
///   - "voter <nom_votant> <candidat>"  (candidat vide = vote blanc)
pub async fn handle_line<Store: Strorage>(
    line: &str,
    controller: &mut VotingController<Store>,
    lexicon: &Lexicon,
) -> anyhow::Result<String> {
    let parts: Vec<&str> = line.trim().splitn(3, ' ').collect();
    let cmd = parts[0];

    match cmd {
        "voter" | "vote" => {
            let voter = parts.get(1).copied().unwrap_or("").to_string();
            if voter.is_empty() {
                return Ok(lexicon.voter_required.to_string());
            }
            let candidat = parts.get(2).copied().unwrap_or("").to_lowercase();

            let form = VoteForm { voter, candidat };
            let outcome = controller.vote(form).await?;
            Ok(show_vote_outcome(outcome, lexicon))
        }
        "votants" | "voters" => {
            let voting_machine = controller.get_voting_machine().await?;
            Ok(show_attendance_sheet(voting_machine.get_voters(), lexicon))
        }
        "scores" => {
            let voting_machine = controller.get_voting_machine().await?;
            Ok(show_scoreboard(voting_machine.get_scoreboard(), lexicon))
        }
        "quit" | "quitter" => {
            std::process::exit(0);
        }
        _ => Ok(format!("{}: {}",lexicon.unknown_command, cmd)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{Candidate, Votingmachine};
    use crate::use_case::VotingController;
    use crate::storages::memory::MemoryStore;
    use crate::interfaces::lexicons::french::FRENCH;

    async fn make_controller() -> VotingController<MemoryStore> {
        let machine = Votingmachine::new(vec![
            Candidate("alice".to_string()),
            Candidate("bob".to_string()),
        ]);
        let store = MemoryStore::new(machine).await.unwrap();
        VotingController::new(store)
    }

    #[tokio::test]
    async fn test_commande_vide_retourne_inconnue() {
        let mut ctrl = make_controller().await;
        let res = handle_line("", &mut ctrl, &FRENCH).await.unwrap();
        assert!(res.contains(FRENCH.unknown_command));
    }

    #[tokio::test]
    async fn test_afficher_scores() {
        let mut ctrl = make_controller().await;
        let res = handle_line("scores", &mut ctrl, &FRENCH).await.unwrap();
        assert!(res.contains(FRENCH.score));
    }

    #[tokio::test]
    async fn test_voter() {
        let mut ctrl = make_controller().await;
        let res = handle_line("voter alice bob", &mut ctrl, &FRENCH).await.unwrap();
        assert!(res.contains("alice"));
        assert!(res.contains("bob"));
        assert!(res.contains(FRENCH.has_voted));
    }

    #[tokio::test]
    async fn test_voter_blanc() {
        let mut ctrl = make_controller().await;
        let res = handle_line("voter alice", &mut ctrl, &FRENCH).await.unwrap();
        assert!(res.contains("alice"));
        assert!(res.contains(FRENCH.blank_vote_registered));
    }

    #[tokio::test]
    async fn test_voter_sans_nom_demande_votant() {
        let mut ctrl = make_controller().await;
        let res = handle_line("voter", &mut ctrl, &FRENCH).await.unwrap();
        assert_eq!(res, FRENCH.voter_required);
    }

    #[tokio::test]
    async fn test_commande_invalide() {
        let mut ctrl = make_controller().await;
        let res = handle_line("xyz", &mut ctrl, &FRENCH).await.unwrap();
        assert!(res.contains(FRENCH.unknown_command));
        assert!(res.contains("xyz"));
    }
}

