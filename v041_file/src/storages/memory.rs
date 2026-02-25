use async_trait::async_trait;

use crate::domain::Votingmachine;
use crate::storage::Strorage;

pub struct MemoryStore {
	machine: Votingmachine,
}

#[async_trait]
impl Strorage for MemoryStore {
	async fn new(machine: Votingmachine) -> anyhow::Result<Self> {
		Ok(Self { machine })
	}

	async fn get_voting_machine(&self) -> anyhow::Result<Votingmachine> {
		Ok(self.machine.clone())
	}

	async fn put_voting_machine(&mut self, machine: Votingmachine) -> anyhow::Result<()> {
		self.machine = machine;
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::domain::Candidate;
	use crate::storage::Strorage;

	#[tokio::test]
	async fn test_get_returns_machine_put_by_put() {
		let initial_machine = Votingmachine::new(vec![Candidate("alice".to_string())]);
		let mut store = MemoryStore::new(initial_machine)
			.await
			.expect("memory store should initialize");

		let updated_machine = Votingmachine::new(vec![Candidate("bob".to_string())]);
		store
			.put_voting_machine(updated_machine.clone())
			.await
			.expect("memory store should accept put");

		let fetched_machine = store
			.get_voting_machine()
			.await
			.expect("memory store should return machine");

		assert_eq!(fetched_machine, updated_machine);
	}
}
