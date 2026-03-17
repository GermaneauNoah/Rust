use async_trait::async_trait;
use tokio::net::TcpListener;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use crate::interfaces::lexicon::Lexicon;
use crate::interfaces::cli_interface;
use crate::service::Service;
use crate::storage::Strorage;
use crate::use_case::VotingController;

pub struct TcpService<Store> {
    port: u16,
    lexicon: Lexicon,
    controller: VotingController<Store>,
}

#[async_trait]
impl<Store: Strorage + Send + Sync + Clone + 'static> Service<Store> for TcpService<Store> {
    fn new(port: u16, lexicon: Lexicon, controller: VotingController<Store>) -> Self {
        Self { port, lexicon, controller }
    }

    async fn serve(&self) -> Result<(), anyhow::Error> {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", self.port)).await?;
        println!("TCP en écoute sur le port {}", self.port);

        loop {
            let (stream, _addr) = listener.accept().await?;
            let lexicon = self.lexicon.clone();
            let controller = self.controller.clone(); // Le clone est maintenant possible grâce au Arc interne

            tokio::spawn(async move {
                let (reader, mut writer) = stream.into_split();
                let mut lines = BufReader::new(reader).lines();

                while let Ok(Some(line)) = lines.next_line().await {
                    if line.trim().is_empty() {
                        continue;
                    }
                    // Note: handle_line ne devrait plus prendre de '&mut' ici
                    match cli_interface::handle_line(&line, &controller, &lexicon).await {
                        Ok(response) => {
                            let _ = writer.write_all(format!("{}\n", response).as_bytes()).await;
                        }
                        Err(e) => {
                            let _ = writer.write_all(format!("Erreur: {}\n", e).as_bytes()).await;
                        }
                    }
                }
            });
        }
    }
}
