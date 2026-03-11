
use v070_cli_services::configuration::charger_configuration;
use v070_cli_services::app_builder::run_app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let configuration = charger_configuration();
    run_app(configuration).await
}
