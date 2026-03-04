use serde::Deserialize;
use crate::domain::{BallotPaper, Voter, Candidate, VoteOutcome, Votingmachine};
use crate::storage::Strorage;

#[derive(Deserialize)]
pub struct VoteForm{
    pub voter: String,
    pub candidat: String,
}

impl From <VoteForm> for BallotPaper {
    fn from(form: VoteForm) -> Self {
        BallotPaper {
            voter: Voter(form.voter),
            choice: if form.candidat.is_empty() {
                None
            } else {
                Some(Candidate(form.candidat))
            },
        }
    }
}
pub struct VotingController<Store>{
    store: Store,
}

impl <Store: Strorage> VotingController<Store> {
    pub fn new(store: Store) -> Self {
        Self { store }
    }

    pub async fn vote(&mut self, form: VoteForm) -> anyhow::Result<VoteOutcome> {
        let ballot_paper: BallotPaper = form.into();
        let mut voting_machine = self.store.get_voting_machine().await?;
        let outcome = voting_machine.vote(ballot_paper);
        self.store.put_voting_machine(voting_machine).await?;
        Ok(outcome)
    }

    pub async fn get_voting_machine(&self) -> anyhow::Result<Votingmachine> {
        self.store.get_voting_machine().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Candidate;
    use crate::storages::memory::MemoryStore;

    async fn make_controller() -> VotingController<MemoryStore> {
        let machine = Votingmachine::new(vec![
            Candidate("alice".to_string()),
            Candidate("bob".to_string()),
        ]);
        let store = MemoryStore::new(machine).await.unwrap();
        VotingController::new(store)
    }

    fn form(voter: &str, candidat: &str) -> VoteForm {
        VoteForm {
            voter: voter.to_string(),
            candidat: candidat.to_string(),
        }
    }

    // Règle 1 : vote accepté pour un candidat valide
    #[tokio::test]
    async fn test_vote_accepte() {
        let mut controller = make_controller().await;

        // Premier vote : charlie vote pour alice
        let outcome = controller.vote(form("charlie", "alice")).await.unwrap();

        let machine = controller.get_voting_machine().await.unwrap();
        let score_alice = machine.get_scoreboard().scores[&Candidate("alice".to_string())];

        assert_eq!(outcome, VoteOutcome::AcceptedVote(Voter("charlie".to_string()), Candidate("alice".to_string())));
        assert_eq!(score_alice.0, 1);
    }

    // Règle 2 : vote blanc si candidat vide
    #[tokio::test]
    async fn test_vote_blanc() {
        let mut controller = make_controller().await;

        // Plusieurs votes avant pour vérifier la persistance du stockage
        controller.vote(form("charlie", "alice")).await.unwrap();
        controller.vote(form("dave", "bob")).await.unwrap();
        let outcome = controller.vote(form("eve", "")).await.unwrap();

        let machine = controller.get_voting_machine().await.unwrap();
        let blank_score = machine.get_scoreboard().blank_score;

        assert_eq!(outcome, VoteOutcome::BlankVote(Voter("eve".to_string())));
        assert_eq!(blank_score.0, 1);
    }

    // Règle 3 : vote nul si candidat inconnu
    #[tokio::test]
    async fn test_vote_nul() {
        let mut controller = make_controller().await;

        // Plusieurs votes avant pour vérifier la persistance du stockage
        controller.vote(form("charlie", "alice")).await.unwrap();
        let outcome = controller.vote(form("dave", "inconnu")).await.unwrap();

        let machine = controller.get_voting_machine().await.unwrap();
        let invalid_score = machine.get_scoreboard().invalid_score;

        assert_eq!(outcome, VoteOutcome::InvalidVote(Voter("dave".to_string())));
        assert_eq!(invalid_score.0, 1);
    }

    // Règle 4 : deuxième vote refusé pour un même votant
    #[tokio::test]
    async fn test_vote_deja_vote() {
        let mut controller = make_controller().await;

        // Premier vote valide
        controller.vote(form("charlie", "alice")).await.unwrap();
        // Deuxième tentative du même votant
        let outcome = controller.vote(form("charlie", "bob")).await.unwrap();

        let machine = controller.get_voting_machine().await.unwrap();
        let score_bob = machine.get_scoreboard().scores[&Candidate("bob".to_string())];

        assert_eq!(outcome, VoteOutcome::HasAlreadyVoted(Voter("charlie".to_string())));
        assert_eq!(score_bob.0, 0);
    }
}
