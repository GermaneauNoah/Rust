use std::collections::BTreeMap as Map;

pub mod configuration;
pub mod app_builder;
pub mod domain;
pub mod storage;
pub mod storages;

// Fonctions métier testables
pub fn initialiser_scores(candidats: &[String]) -> Map<String, i32> {
    Map::from_iter(candidats.iter().map(|c| (c.to_string(), 0)))
}

pub fn obtenir_candidats(configuration: &configuration::Configuration) -> Vec<String> {
    configuration.candidats_reels()
}

pub fn est_candidat_valide(candidat: &str, candidats: &[String]) -> bool {
    candidats.iter().any(|c| c == &candidat.to_lowercase())
}

pub fn enregistrer_vote(
    candidat: &str,
    scores: &mut Map<String, i32>,
    votes_blancs: &mut i32,
    votes_nuls: &mut i32,
    candidats: &[String],
) -> String {
    let candidat_lower = candidat.trim().to_lowercase();
    
    if candidat_lower.is_empty() {
        *votes_blancs += 1;
        "blanc".to_string()
    } else if est_candidat_valide(&candidat_lower, candidats) {
        if let Some(score) = scores.get_mut(&candidat_lower) {
            *score += 1;
        }
        format!("valid:{}", candidat_lower)
    } else {
        *votes_nuls += 1;
        "nul".to_string()
    }
}
