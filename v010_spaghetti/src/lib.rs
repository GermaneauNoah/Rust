use std::collections::BTreeMap as Map;

// Fonctions métier testables
pub fn initialiser_scores() -> Map<String, i32> {
    Map::from([
        ("alice".to_string(), 0),
        ("bob".to_string(), 0),
        ("charlie".to_string(), 0),
        ("bill".to_string(), 0),
        ("bao".to_string(), 0),
    ])
}

pub fn obtenir_candidats() -> Vec<String> {
    vec!["alice".to_string(), "bob".to_string(), "charlie".to_string(), "bill".to_string(), "bao".to_string()]
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
