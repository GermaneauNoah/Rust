use v010_spaghetti::{initialiser_scores, obtenir_candidats, est_candidat_valide, enregistrer_vote};

#[test]
fn test_initialiser_scores() {
    let scores = initialiser_scores();
    assert_eq!(scores.len(), 5);
    assert_eq!(scores.get("alice"), Some(&0));
    assert_eq!(scores.get("bob"), Some(&0));
    assert_eq!(scores.get("charlie"), Some(&0));
    assert_eq!(scores.get("bill"), Some(&0));
    assert_eq!(scores.get("bao"), Some(&0));
}

#[test]
fn test_obtenir_candidats() {
    let candidats = obtenir_candidats();
    assert_eq!(candidats.len(), 5);
    assert!(candidats.contains(&"alice".to_string()));
    assert!(candidats.contains(&"bob".to_string()));
    assert!(candidats.contains(&"charlie".to_string()));
    assert!(candidats.contains(&"bill".to_string()));
    assert!(candidats.contains(&"bao".to_string()));
}

#[test]
fn test_est_candidat_valide() {
    let candidats = obtenir_candidats();
    assert!(est_candidat_valide("alice", &candidats));
    assert!(est_candidat_valide("ALICE", &candidats)); // Test insensibilité à la casse
    assert!(est_candidat_valide("Bob", &candidats));
    assert!(!est_candidat_valide("unknown", &candidats));
    assert!(!est_candidat_valide("", &candidats));
}

#[test]
fn test_enregistrer_vote_valide() {
    let mut scores = initialiser_scores();
    let mut votes_blancs = 0;
    let mut votes_nuls = 0;
    let candidats = obtenir_candidats();

    let resultat = enregistrer_vote("alice", &mut scores, &mut votes_blancs, &mut votes_nuls, &candidats);

    assert_eq!(resultat, "valid:alice");
    assert_eq!(scores.get("alice"), Some(&1));
    assert_eq!(votes_blancs, 0);
    assert_eq!(votes_nuls, 0);
}

#[test]
fn test_enregistrer_vote_blanc() {
    let mut scores = initialiser_scores();
    let mut votes_blancs = 0;
    let mut votes_nuls = 0;
    let candidats = obtenir_candidats();

    let resultat = enregistrer_vote("", &mut scores, &mut votes_blancs, &mut votes_nuls, &candidats);

    assert_eq!(resultat, "blanc");
    assert_eq!(votes_blancs, 1);
    assert_eq!(votes_nuls, 0);
    assert_eq!(scores.get("alice"), Some(&0)); // Les scores ne changent pas
}

#[test]
fn test_enregistrer_vote_nul() {
    let mut scores = initialiser_scores();
    let mut votes_blancs = 0;
    let mut votes_nuls = 0;
    let candidats = obtenir_candidats();

    let resultat = enregistrer_vote("candidat_inexistant", &mut scores, &mut votes_blancs, &mut votes_nuls, &candidats);

    assert_eq!(resultat, "nul");
    assert_eq!(votes_blancs, 0);
    assert_eq!(votes_nuls, 1);
    assert_eq!(scores.get("alice"), Some(&0)); // Les scores ne changent pas
}

#[test]
fn test_enregistrer_vote_insensible_casse() {
    let mut scores = initialiser_scores();
    let mut votes_blancs = 0;
    let mut votes_nuls = 0;
    let candidats = obtenir_candidats();

    enregistrer_vote("CHARLIE", &mut scores, &mut votes_blancs, &mut votes_nuls, &candidats);
    enregistrer_vote("BoB", &mut scores, &mut votes_blancs, &mut votes_nuls, &candidats);

    assert_eq!(scores.get("charlie"), Some(&1));
    assert_eq!(scores.get("bob"), Some(&1));
}

#[test]
fn test_multiple_votes() {
    let mut scores = initialiser_scores();
    let mut votes_blancs = 0;
    let mut votes_nuls = 0;
    let candidats = obtenir_candidats();

    enregistrer_vote("alice", &mut scores, &mut votes_blancs, &mut votes_nuls, &candidats);
    enregistrer_vote("alice", &mut scores, &mut votes_blancs, &mut votes_nuls, &candidats);
    enregistrer_vote("bob", &mut scores, &mut votes_blancs, &mut votes_nuls, &candidats);
    enregistrer_vote("", &mut scores, &mut votes_blancs, &mut votes_nuls, &candidats);
    enregistrer_vote("invalid", &mut scores, &mut votes_blancs, &mut votes_nuls, &candidats);

    assert_eq!(scores.get("alice"), Some(&2));
    assert_eq!(scores.get("bob"), Some(&1));
    assert_eq!(votes_blancs, 1);
    assert_eq!(votes_nuls, 1);
}
