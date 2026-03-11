use async_trait::async_trait;
use tokio::io::{AsyncBufReadExt, BufReader};
use crate::interfaces::lexicon::Lexicon;
use crate::interfaces::cli_interface;
use crate::service::Service;
use crate::storage::Strorage;
use crate::use_case::VotingController;

pub struct StdioService<Store> {
    lexicon: Lexicon,
    controller: VotingController<Store>,
}

#[async_trait]
impl<Store: Strorage + Send + Sync> Service<Store> for StdioService<Store> {
    fn new(_port: u16, lexicon: Lexicon, controller: VotingController<Store>) -> Self {
        Self { lexicon, controller }
    }

    async fn serv(&mut self) -> Result<(), anyhow::Error> {
        let stdin = tokio::io::stdin();
        let reader = BufReader::new(stdin);
        let mut lines = reader.lines();

        loop {
            println!("{}", self.lexicon.command);

            let Some(line) = lines.next_line().await? else {
                break;
            };

            if line.trim().is_empty() {
                continue;
            }

            let response = cli_interface::handle_line(&line, &mut self.controller, &self.lexicon).await?;
            println!("{}", response);
        }

        Ok(())
    }
}
