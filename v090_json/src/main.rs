
use v090_json::configuration::charger_configuration;
use v090_json::app_builder::run_app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let configuration = charger_configuration();
    run_app(configuration).await
}