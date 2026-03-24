use async_trait::async_trait;
use crate::interfaces::lexicon::Lexicon;
use crate::use_case::VotingController;

#[async_trait]
pub trait Service<Store> {
    fn new(port: u16, lexicon: Lexicon, controller: VotingController<Store>) -> Self;
    async fn serve(&self) -> Result<(), anyhow::Error>;
}
