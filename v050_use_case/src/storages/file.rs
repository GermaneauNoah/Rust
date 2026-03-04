use std::collections::BTreeMap as Map;
use std::collections::BTreeSet as Set;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;

use crate::domain::{AttendencesSheet, Candidate, Score, Scoreboard, Voter, Votingmachine};
use crate::storage::Strorage;

pub const FILEPATH: &str = "machine.json";

pub struct FileStore {
	pub filepath: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ScoreboardDao {
	scores: Map<String, usize>,
	blank_score: usize,
	invalid_score: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct VotingmachineDao {
	voters: Set<String>,
	scoreboard: ScoreboardDao,
}

impl From<Scoreboard> for ScoreboardDao {
	fn from(scoreboard: Scoreboard) -> Self {
		let Scoreboard {
			scores,
			blank_score,
			invalid_score,
		} = scoreboard;

		let scores = scores
			.into_iter()
			.map(|(candidate, score)| (candidate.0, score.0))
			.collect();

		Self {
			scores,
			blank_score: blank_score.0,
			invalid_score: invalid_score.0,
		}
	}
}

impl From<ScoreboardDao> for Scoreboard {
	fn from(scoreboard_dao: ScoreboardDao) -> Self {
		let scores = scoreboard_dao
			.scores
			.into_iter()
			.map(|(candidate, score)| (Candidate(candidate), Score(score)))
			.collect();

		Self {
			scores,
			blank_score: Score(scoreboard_dao.blank_score),
			invalid_score: Score(scoreboard_dao.invalid_score),
		}
	}
}

impl From<Votingmachine> for VotingmachineDao {
	fn from(machine: Votingmachine) -> Self {
		let voters = machine
			.get_voters()
			.voters()
			.iter()
			.map(|voter| voter.0.clone())
			.collect();

		Self {
			voters,
			scoreboard: machine.get_scoreboard().clone().into(),
		}
	}
}

impl From<VotingmachineDao> for Votingmachine {
	fn from(machine_dao: VotingmachineDao) -> Self {
		let voters = AttendencesSheet(
			machine_dao
				.voters
				.into_iter()
				.map(Voter)
				.collect::<Set<Voter>>(),
		);

		let scoreboard: Scoreboard = machine_dao.scoreboard.into();
		Votingmachine::recover_from(voters, scoreboard)
	}
}

impl FileStore {
	pub async fn create(machine: Votingmachine, filepath: &str) -> anyhow::Result<Self> {
		if !tokio::fs::try_exists(filepath).await? {
			store_voting_machine(machine, filepath).await?;
		}

		Ok(Self {
			filepath: filepath.to_string(),
		})
	}
}

#[async_trait]
impl Strorage for FileStore {
	async fn new(machine: Votingmachine) -> anyhow::Result<Self> {
		Self::create(machine, FILEPATH).await
	}

	async fn get_voting_machine(&self) -> anyhow::Result<Votingmachine> {
		load_voting_machine(&self.filepath).await
	}

	async fn put_voting_machine(&mut self, machine: Votingmachine) -> anyhow::Result<()> {
		store_voting_machine(machine, &self.filepath).await?;
		Ok(())
	}
}

async fn store_voting_machine(machine: Votingmachine, filepath: &str) -> anyhow::Result<()> {
	let dao: VotingmachineDao = machine.into();
	let serialized = serde_json::to_string_pretty(&dao)?;
	let mut file = tokio::fs::OpenOptions::new()
		.create(true)
		.write(true)
		.truncate(true)
		.open(filepath)
		.await?;
	file.write_all(serialized.as_bytes()).await?;
	file.flush().await?;
	Ok(())
}

async fn load_voting_machine(filepath: &str) -> anyhow::Result<Votingmachine> {
	let content = tokio::fs::read_to_string(filepath).await?;
	let dao: VotingmachineDao = serde_json::from_str(&content)?;
	Ok(dao.into())
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::storage::Strorage;

	/// Vérifie que la machine récupérée par `get_voting_machine` est identique
	/// à celle stockée par `put_voting_machine`.
	///
	/// Note : `voters_who_voted` n'est pas persisté dans le DAO ; la machine
	/// initiale est donc construite via `recover_from` (qui initialise ce champ
	/// à vide), ce qui garantit la cohérence de la comparaison après round-trip.
	#[tokio::test]
	async fn test_get_retourne_la_machine_stockee_par_put() {
		let filepath = "test_roundtrip_put_get.json";

		// --- Arrange ---
		let mut scores = Map::new();
		scores.insert(Candidate("alice".to_string()), Score(3));
		scores.insert(Candidate("bob".to_string()), Score(1));

		let scoreboard = Scoreboard {
			scores,
			blank_score: Score(1),
			invalid_score: Score(0),
		};

		let mut voters = AttendencesSheet::new();
		voters.add_voter(Voter("jean".to_string()));
		voters.add_voter(Voter("marie".to_string()));
		voters.add_voter(Voter("pierre".to_string()));
		voters.add_voter(Voter("sophie".to_string()));
		voters.add_voter(Voter("paul".to_string()));

		// `recover_from` initialise `voters_who_voted` à vide, comme le ferait
		// une désérialisation depuis le fichier.
		let machine = Votingmachine::recover_from(voters, scoreboard);

		let mut store = FileStore {
			filepath: filepath.to_string(),
		};
		store.put_voting_machine(machine.clone()).await.unwrap();

		// --- Act ---
		let loaded = store.get_voting_machine().await.unwrap();

		// --- Assert ---
		assert_eq!(machine, loaded);

		// Nettoyage du fichier temporaire
		let _ = tokio::fs::remove_file(filepath).await;
	}

	/// Vérifie que les données écrites par une instance de `FileStore` sont
	/// bien retrouvées par une seconde instance indépendante pointant sur le
	/// même fichier.
	#[tokio::test]
	async fn test_donnees_persistees_entre_deux_instances_filestore() {
		let filepath = "test_persistence_deux_instances.json";

		// --- Arrange ---
		let mut scores = Map::new();
		scores.insert(Candidate("charlie".to_string()), Score(5));
		scores.insert(Candidate("diana".to_string()), Score(2));

		let scoreboard = Scoreboard {
			scores,
			blank_score: Score(0),
			invalid_score: Score(1),
		};

		let mut voters = AttendencesSheet::new();
		voters.add_voter(Voter("luc".to_string()));
		voters.add_voter(Voter("emma".to_string()));

		let machine = Votingmachine::recover_from(voters, scoreboard);

		// Première instance : écriture puis abandon de l'instance
		{
			let mut store1 = FileStore {
				filepath: filepath.to_string(),
			};
			store1.put_voting_machine(machine.clone()).await.unwrap();
		} // store1 est dropped ici

		// Deuxième instance : nouvelle instanciation indépendante
		let store2 = FileStore {
			filepath: filepath.to_string(),
		};

		// --- Act ---
		let loaded = store2.get_voting_machine().await.unwrap();

		// --- Assert ---
		assert_eq!(machine, loaded);

		// Nettoyage du fichier temporaire
		let _ = tokio::fs::remove_file(filepath).await;
	}
}