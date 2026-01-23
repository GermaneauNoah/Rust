
use std::collections::{BTreeMap as Map, HashSet};
use tokio::io::{AsyncBufReadExt, BufReader};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let stdin = tokio::io::stdin();
    let reader = BufReader::new(stdin);
    let mut lines = reader.lines();
    let mut votants: Vec<String> = Vec::new();
    let mut scores: Map<String, i32> = Map::from([
        ("alice".to_string(), 0),
        ("bob".to_string(), 0),
        ("charlie".to_string(), 0),
        ("bill".to_string(), 0),
        ("bao".to_string(), 0),
    ]);
    let candidats: Vec<String> = vec!["alice".to_string(), "bob".to_string(), "charlie".to_string(), "bill".to_string(), "bao".to_string()];
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
                            let candidat = candidat_line.trim().to_lowercase();
                            
                            if candidat.is_empty() {
                                votes_blancs += 1;
                                println!("Vote blanc enregistré pour {}.", nom_votant);
                            } else if scores.contains_key(&candidat) {
                                if let Some(score) = scores.get_mut(&candidat) {
                                    *score += 1;
                                }
                                println!("Vote pour {} enregistré pour {}.", candidat, nom_votant);
                            } else {
                                votes_nuls += 1;
                                println!("Vote nul enregistré pour {}.", nom_votant);
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
