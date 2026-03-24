use crate::configuration::{Configuration, StorageType, LanguageType, ServiceType};
use crate::domain::{Votingmachine, Candidate};
use crate::storage::Strorage;
use crate::storages::file::FileStore;
use crate::storages::memory::MemoryStore;
use crate::use_case::VotingController;
use crate::service::Service;
use crate::services::stdio::StdioService;
use crate::services::udp::UdpService;
use crate::services::web::WebService;
use crate::interfaces::lexicons::french::FRENCH;
use crate::interfaces::lexicons::english::ENGLISH;

fn create_voting_machine(configuration: &Configuration) -> Votingmachine {
    let candidats: Vec<Candidate> = configuration.candidats_reels()
        .into_iter()
        .map(|c| Candidate(c))
        .collect();
    Votingmachine::new(candidats)
}

pub async fn handle_lines<Store: Strorage + Send + Sync + Clone + 'static, Serv: Service<Store>>(configuration: Configuration) -> anyhow::Result<()> {
    let initial_machine = create_voting_machine(&configuration);
    let store = Store::new(initial_machine).await?;
    let controller = VotingController::new(store);
    let lexicon = match configuration.language() {
        LanguageType::Fr => FRENCH,
        LanguageType::En => ENGLISH,
    };
    let port = configuration.port();

    Serv::new(port, lexicon, controller).serve().await
}

pub async fn run_app(configuration: Configuration) -> anyhow::Result<()> {
    match configuration.storage() {
        StorageType::File => dispatch_service::<FileStore>(configuration).await,
        StorageType::Memory => dispatch_service::<MemoryStore>(configuration).await,
    }
}

async fn dispatch_service<Store : Strorage + Send + Sync + Clone + 'static>(configuration: Configuration) -> Result<(), anyhow::Error> {
    match configuration.service() {
        ServiceType::Stdio => handle_lines::<Store, StdioService<Store>>(configuration).await,
        ServiceType::Udp => handle_lines::<Store, UdpService<Store>>(configuration).await,
        ServiceType::Web => handle_lines::<Store, WebService<Store>>(configuration).await,
    }
}
