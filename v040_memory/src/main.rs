
use v030_domain::configuration::charger_configuration;
use v030_domain::app_builder::run_app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let configuration = charger_configuration();
    run_app(configuration).await
}
