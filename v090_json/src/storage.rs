use async_trait::async_trait;

use crate::domain::Votingmachine;

#[async_trait]
pub trait Strorage where Self: Sized {
    async fn new(machine: Votingmachine) -> anyhow::Result<Self>;
    async fn get_voting_machine(&self) -> anyhow::Result<Votingmachine>;
    async fn put_voting_machine(&mut self, machine: Votingmachine) -> anyhow::Result<()>;
}