
use v021_app_builder::configuration::charger_configuration;
use v021_app_builder::app_builder::run_app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let configuration = charger_configuration();
    run_app(configuration).await
}
