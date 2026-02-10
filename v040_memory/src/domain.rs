use std::collections::BTreeMap as Map;
use std::collections::BTreeSet as Set;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Voter(pub String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Candidate(pub String);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Score(pub usize);

pub struct AttendencesSheet(pub Set<Voter>);

pub struct Scoreboard{
    pub scores: Map<Candidate, Score>,
    pub blank_score: Score,
    pub invalid_score: Score,
}

pub struct BallotPaper {
    pub voter: Voter,
    pub choice: Option<Candidate>,
}

pub enum VoteOutcome {
    AcceptedVote(Voter, Candidate),
    BlankVote(Voter),
    InvalidVote(Voter),
    HasAlreadyVoted(Voter),
}

pub struct Votingmachine {
    voters: AttendencesSheet,
    voters_who_voted: Set<Voter>,
    scoreboard: Scoreboard,
    valid_candidates: Set<Candidate>,
}

impl Scoreboard {
    pub fn new(candidates: Vec<Candidate>) -> Self {
        let scores = candidates.into_iter()
            .map(|c| (c, Score(0)))
            .collect();
        Self {
            scores,
            blank_score: Score(0),
            invalid_score: Score(0),
        }
    }
}

impl AttendencesSheet {
    pub fn new() -> Self {
        Self(Set::new())
    }

    pub fn add_voter(&mut self, voter: Voter) {
        self.0.insert(voter);
    }

    pub fn voters(&self) -> &Set<Voter> {
        &self.0
    }
}

impl Votingmachine {
    pub fn new(candidates: Vec<Candidate>) -> Self {
        let valid_candidates: Set<Candidate> = candidates.iter().cloned().collect();
        let scoreboard = Scoreboard::new(candidates);
        Self {
            voters: AttendencesSheet::new(),
            voters_who_voted: Set::new(),
            scoreboard,
            valid_candidates,
        }
    }

    pub fn get_scoreboard(&self) -> &Scoreboard {
        &self.scoreboard
    }

    pub fn get_voters(&self) -> &AttendencesSheet {
        &self.voters
    }

    pub fn vote(&mut self, ballot_paper: BallotPaper) -> VoteOutcome {
        // Ajouter le votant à la feuille de présence
        self.voters.add_voter(ballot_paper.voter.clone());

        // Vérifier si le votant a déjà voté
        if self.voters_who_voted.contains(&ballot_paper.voter) {
            return VoteOutcome::HasAlreadyVoted(ballot_paper.voter);
        }

        // Enregistrer que le votant a voté
        self.voters_who_voted.insert(ballot_paper.voter.clone());

        // Traiter le vote
        match ballot_paper.choice {
            None => {
                // Vote blanc
                self.scoreboard.blank_score.0 += 1;
                VoteOutcome::BlankVote(ballot_paper.voter)
            }
            Some(candidate) => {
                // Vérifier si le candidat est valide
                if self.valid_candidates.contains(&candidate) {
                    // Vote valide
                    if let Some(score) = self.scoreboard.scores.get_mut(&candidate) {
                        score.0 += 1;
                    }
                    VoteOutcome::AcceptedVote(ballot_paper.voter, candidate)
                } else {
                    // Vote nul
                    self.scoreboard.invalid_score.0 += 1;
                    VoteOutcome::InvalidVote(ballot_paper.voter)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_voter_ne_peut_pas_voter_plusieurs_fois() {
        // Arrange
        let candidats = vec![Candidate("alice".to_string()), Candidate("bob".to_string())];
        let mut machine = Votingmachine::new(candidats);
        let votant = Voter("jean".to_string());
        
        // Premier vote
        let bulletin1 = BallotPaper {
            voter: votant.clone(),
            choice: Some(Candidate("alice".to_string())),
        };
        let resultat1 = machine.vote(bulletin1);
        
        // Act - Tentative de second vote
        let bulletin2 = BallotPaper {
            voter: votant.clone(),
            choice: Some(Candidate("bob".to_string())),
        };
        let resultat2 = machine.vote(bulletin2);
        
        // Assert
        assert!(matches!(resultat1, VoteOutcome::AcceptedVote(_, _)));
        assert!(matches!(resultat2, VoteOutcome::HasAlreadyVoted(_)));
        
        // Vérifier que seul le premier vote a été comptabilisé
        let scoreboard = machine.get_scoreboard();
        assert_eq!(scoreboard.scores.get(&Candidate("alice".to_string())).unwrap().0, 1);
        assert_eq!(scoreboard.scores.get(&Candidate("bob".to_string())).unwrap().0, 0);
    }


    
    #[test]
    fn test_vote_pour_candidat_inconnu_est_nul() {
        // Arrange
        let candidats = vec![Candidate("alice".to_string()), Candidate("bob".to_string())];
        let mut machine = Votingmachine::new(candidats);
        
        // Act - Vote pour un candidat qui n'existe pas
        let bulletin = BallotPaper {
            voter: Voter("marie".to_string()),
            choice: Some(Candidate("charlie".to_string())),
        };
        let resultat = machine.vote(bulletin);
        
        // Assert
        assert!(matches!(resultat, VoteOutcome::InvalidVote(_)));
        
        // Vérifier que le vote nul a été comptabilisé
        let scoreboard = machine.get_scoreboard();
        assert_eq!(scoreboard.invalid_score.0, 1);
    }

    #[test]
    fn test_vote_blanc_avec_aucun_candidat() {
        // Arrange
        let candidats = vec![Candidate("alice".to_string()), Candidate("bob".to_string())];
        let mut machine = Votingmachine::new(candidats);
        
        // Act - Vote blanc (choice = None)
        let bulletin = BallotPaper {
            voter: Voter("pierre".to_string()),
            choice: None,
        };
        let resultat = machine.vote(bulletin);
        
        // Assert
        assert!(matches!(resultat, VoteOutcome::BlankVote(_)));
        
        // Vérifier que le vote blanc a été comptabilisé
        let scoreboard = machine.get_scoreboard();
        assert_eq!(scoreboard.blank_score.0, 1);
    }

    #[test]
    fn test_vote_valide_incremente_score_candidat() {
        // Arrange
        let candidats = vec![Candidate("alice".to_string()), Candidate("bob".to_string())];
        let mut machine = Votingmachine::new(candidats);
        
        // Act - Vote valide pour Alice
        let bulletin = BallotPaper {
            voter: Voter("sophie".to_string()),
            choice: Some(Candidate("alice".to_string())),
        };
        let resultat = machine.vote(bulletin);
        
        // Assert
        assert!(matches!(resultat, VoteOutcome::AcceptedVote(_, _)));
        
        // Vérifier que le score d'Alice a été incrémenté
        let scoreboard = machine.get_scoreboard();
        assert_eq!(scoreboard.scores.get(&Candidate("alice".to_string())).unwrap().0, 1);
        assert_eq!(scoreboard.scores.get(&Candidate("bob".to_string())).unwrap().0, 0);
        assert_eq!(scoreboard.blank_score.0, 0);
        assert_eq!(scoreboard.invalid_score.0, 0);
    }
}

