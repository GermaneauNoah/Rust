
use v041_file::configuration::charger_configuration;
use v041_file::app_builder::run_app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let configuration = charger_configuration();
    run_app(configuration).await
}
