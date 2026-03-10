use tokio::io::{AsyncBufReadExt, BufReader};
use crate::configuration::{Configuration, StorageType, LanguageType};
use crate::domain::{Votingmachine, Candidate};
use crate::storage::Strorage;
use crate::storages::file::FileStore;
use crate::storages::memory::MemoryStore;
use crate::use_case::VotingController;
use crate::interfaces::cli_interface;
use crate::interfaces::lexicons::french::FRENCH;
use crate::interfaces::lexicons::english::ENGLISH;

fn create_voting_machine(configuration: &Configuration) -> Votingmachine {
    let candidats: Vec<Candidate> = configuration.candidats_reels()
        .into_iter()
        .map(|c| Candidate(c))
        .collect();
    Votingmachine::new(candidats)
}

pub async fn handle_lines<Store: Strorage>(configuration: Configuration) -> anyhow::Result<()> {
    let stdin = tokio::io::stdin();
    let reader = BufReader::new(stdin);
    let mut lines = reader.lines();

    let initial_machine = create_voting_machine(&configuration);
    let store = Store::new(initial_machine).await?;
    let mut controller = VotingController::new(store);
    let lexicon = match configuration.language() {
        LanguageType::Fr => &FRENCH,
        LanguageType::En => &ENGLISH,
    };

    loop {
        println!("{}", lexicon.command);

        let Some(line) = lines.next_line().await? else {
            break;
        };

        if line.trim().is_empty() {
            continue;
        }

        let response = cli_interface::handle_line(&line, &mut controller, lexicon).await?;
        println!("{}", response);
    }

    Ok(())
}

pub async fn run_app(configuration: Configuration) -> anyhow::Result<()> {
    match configuration.storage() {
        StorageType::File => handle_lines::<FileStore>(configuration).await,
        StorageType::Memory => handle_lines::<MemoryStore>(configuration).await,
    }
}
