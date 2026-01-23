
use std::collections::HashSet;
use tokio::io::{AsyncBufReadExt, BufReader};
use v010_spaghetti::{initialiser_scores, obtenir_candidats, enregistrer_vote};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let stdin = tokio::io::stdin();
    let reader = BufReader::new(stdin);
    let mut lines = reader.lines();
    let mut votants: Vec<String> = Vec::new();
    let mut scores = initialiser_scores();
    let candidats = obtenir_candidats();
    let mut votants_ayant_votes: HashSet<String> = HashSet::new();
    let mut votes_blancs = 0;
    let mut votes_nuls = 0;
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
                    if !votants.contains(&nom_votant) {
                        votants.push(nom_votant.clone());
                    }

                    if votants_ayant_votes.contains(&nom_votant) {
                        println!("Erreur: {} a déjà voté!", nom_votant);
                    } else {
                        println!("Quel candidat choisissez-vous? (laisser vide pour vote blanc)");
                        println!("{}", candidats.join(", "));
                        if let Some(candidat_line) = lines.next_line().await? {
                            let resultat = enregistrer_vote(
                                candidat_line.trim(),
                                &mut scores,
                                &mut votes_blancs,
                                &mut votes_nuls,
                                &candidats,
                            );
                            
                            match resultat.as_str() {
                                "blanc" => println!("Vote blanc enregistré pour {}.", nom_votant),
                                "nul" => println!("Vote nul enregistré pour {}.", nom_votant),
                                s if s.starts_with("valid:") => {
                                    let candidat = &s[6..];
                                    println!("Vote pour {} enregistré pour {}.", candidat, nom_votant);
                                }
                                _ => {}
                            }
                            
                            votants_ayant_votes.insert(nom_votant);
                        }
                    }
                }
            }
            "votants" => {
                println!("");
                println!("-------------------");
                println!("Liste des votants:");
                for nom in &votants {
                    println!("- {}", nom);
                }
                println!("-------------------");
                println!("");
            }
            "scores" => {
                println!("");
                println!("-------------------");
                println!("Scores:");
                for (nom, score) in &scores {
                    println!("{}: {}", nom, score);
                }
                println!("Votes blancs: {}", votes_blancs);
                println!("Votes nuls: {}", votes_nuls);
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
