
use v060_text_interfaces::configuration::charger_configuration;
use v060_text_interfaces::app_builder::run_app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let configuration = charger_configuration();
    run_app(configuration).await
}
