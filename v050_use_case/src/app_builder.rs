use tokio::io::{AsyncBufReadExt, BufReader};
use crate::configuration::{Configuration, StorageType};
use crate::domain::{Votingmachine, Candidate, VoteOutcome};
use crate::storage::Strorage;
use crate::storages::file::FileStore;
use crate::storages::memory::MemoryStore;
use crate::use_case::{VotingController, VoteForm};

fn create_voting_machine(configuration: &Configuration) -> Votingmachine {
    let candidats: Vec<Candidate> = configuration.candidats_reels()
        .into_iter()
        .map(|c| Candidate(c))
        .collect();
    Votingmachine::new(candidats)
}

pub async fn handle_lines<Store: Strorage>(configuration: Configuration) -> anyhow::Result<()> {
    let stdin = tokio::io::stdin();
    let reader = BufReader::new(stdin);
    let mut lines = reader.lines();

    let initial_machine = create_voting_machine(&configuration);
    let store = Store::new(initial_machine).await?;
    let mut controller = VotingController::new(store);
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

                    println!("Quel candidat choisissez-vous? (laisser vide pour vote blanc)");
                    println!("{}", candidats_affichage.join(", "));

                    if let Some(candidat_line) = lines.next_line().await? {
                        let form = VoteForm {
                            voter: nom_votant,
                            candidat: candidat_line.trim().to_lowercase(),
                        };

                        let outcome = controller.vote(form).await?;

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
                let voting_machine = controller.get_voting_machine().await?;
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
                let voting_machine = controller.get_voting_machine().await?;
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

pub async fn run_app(configuration: Configuration) -> anyhow::Result<()> {
    match configuration.storage() {
        StorageType::File => handle_lines::<FileStore>(configuration).await,
        StorageType::Memory => handle_lines::<MemoryStore>(configuration).await,
    }
}
