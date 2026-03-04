
use v050_use_case::configuration::charger_configuration;
use v050_use_case::app_builder::run_app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let configuration = charger_configuration();
    run_app(configuration).await
}
