use tokio::io::{AsyncBufReadExt, BufReader};
use crate::configuration::Configuration;
use crate::domain::{Votingmachine, Candidate, Voter, BallotPaper, VoteOutcome};

fn create_voting_machine(configuration: &Configuration) -> Votingmachine {
    let candidats: Vec<Candidate> = configuration.candidats_reels()
        .into_iter()
        .map(|c| Candidate(c))
        .collect();
    Votingmachine::new(candidats)
}

pub async fn run_app(configuration: Configuration) -> anyhow::Result<()> {
    let stdin = tokio::io::stdin();
    let reader = BufReader::new(stdin);
    let mut lines = reader.lines();
    
    let mut voting_machine = create_voting_machine(&configuration);
    let candidats_affichage = configuration.candidats_affichage();
    let mut continuer = true;
    
    while continuer {
        println!("Saisis une commande (voter, votants, scores):");

        let Some(line) = lines.next_line().await? else {
            break;
        };

        let cmd = line.trim();
        let cmd = if cmd.is_empty() {
            println!("Saisis une commande (voter, votants, scores):");
            let Some(next_cmd) = lines.next_line().await? else {
                break;
            };
            next_cmd.trim().to_owned()
        } else {
            cmd.to_owned()
        };

        match cmd.as_str() {
            "voter" => {
                println!("Quel est votre nom de votant?");
                if let Some(nom_votant_line) = lines.next_line().await? {
                    let nom_votant = nom_votant_line.trim().to_string();
                    let voter = Voter(nom_votant.clone());

                    println!("Quel candidat choisissez-vous? (laisser vide pour vote blanc)");
                    println!("{}", candidats_affichage.join(", "));
                    
                    if let Some(candidat_line) = lines.next_line().await? {
                        let candidat_input = candidat_line.trim();
                        
                        // Créer le bulletin de vote
                        let ballot_paper = if candidat_input.is_empty() {
                            BallotPaper {
                                voter,
                                choice: None,
                            }
                        } else {
                            BallotPaper {
                                voter,
                                choice: Some(Candidate(candidat_input.to_lowercase())),
                            }
                        };
                        
                        // Enregistrer le vote
                        let outcome = voting_machine.vote(ballot_paper);
                        
                        match outcome {
                            VoteOutcome::BlankVote(v) => {
                                println!("Vote blanc enregistré pour {}.", v.0);
                            }
                            VoteOutcome::InvalidVote(v) => {
                                println!("Vote nul enregistré pour {}.", v.0);
                            }
                            VoteOutcome::AcceptedVote(v, c) => {
                                println!("Vote pour {} enregistré pour {}.", c.0, v.0);
                            }
                            VoteOutcome::HasAlreadyVoted(v) => {
                                println!("Erreur: {} a déjà voté!", v.0);
                            }
                        }
                    }
                }
            }
            "votants" => {
                println!("");
                println!("-------------------");
                println!("Liste des votants:");
                for voter in voting_machine.get_voters().voters() {
                    println!("- {}", voter.0);
                }
                println!("-------------------");
                println!("");
            }
            "scores" => {
                let scoreboard = voting_machine.get_scoreboard();
                println!("");
                println!("-------------------");
                println!("Scores:");
                println!("blanc: {}", scoreboard.blank_score.0);
                println!("nul: {}", scoreboard.invalid_score.0);
                for (candidat, score) in &scoreboard.scores {
                    println!("{}: {}", candidat.0, score.0);
                }
                println!("-------------------");
                println!("");
            }
            _ => println!("commande inconnue"),
        }

        println!("voulez-vous continuer? (o/n): ");
        let mut response_valid = false;
        while !response_valid {
            if let Some(answer) = lines.next_line().await? {
                let ans = answer.trim();
                if ans.eq_ignore_ascii_case("n") || ans.eq_ignore_ascii_case("o") {
                    response_valid = true;
                    if ans.eq_ignore_ascii_case("n") {
                        continuer = false;
                    }
                } else {
                    println!("Réponse invalide, entrez 'o' ou 'n': ");
                }
            } else {
                break;
            }
        }
    }

    Ok(())
}