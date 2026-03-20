use async_trait::async_trait;
use tokio::net::UdpSocket;
use crate::interfaces::lexicon::Lexicon;
use crate::interfaces::cli_interface;
use crate::service::Service;
use crate::storage::Strorage;
use crate::use_case::VotingController;

pub struct UdpService<Store> {
    port: u16,
    lexicon: Lexicon,
    controller: VotingController<Store>,
}

#[async_trait]
impl<Store: Strorage + Send + Sync> Service<Store> for UdpService<Store> {
    fn new(port: u16, lexicon: Lexicon, controller: VotingController<Store>) -> Self {
        Self { port, lexicon, controller }
    }

    async fn serve(&self) -> Result<(), anyhow::Error> {
        let socket = UdpSocket::bind(format!("0.0.0.0:{}", self.port)).await?;
        println!("UDP en écoute sur le port {}", self.port);

        let mut buf = vec![0u8; 1024];

        loop {
            let (len, addr) = socket.recv_from(&mut buf).await?;
            let line = String::from_utf8_lossy(&buf[..len]).to_string();

            if line.trim().is_empty() {
                continue;
            }

            let response = cli_interface::handle_line(&line, &self.controller, &self.lexicon).await?;
            socket.send_to(response.as_bytes(), addr).await?;
        }
    }
}
