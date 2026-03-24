use std::net::SocketAddr;
use std::marker::PhantomData;
use async_trait::async_trait;
use axum::Router;

use crate::service::Service;
use crate::storage::Strorage;
use crate::use_case::VotingController;
use crate::interfaces::lexicon::Lexicon;
use crate::interfaces::web_interfaces::{AxumState, router::make_router, web_routes::WEB_ROUTES};

pub struct WebService<Store> {
    address: SocketAddr,
    router: Router,
    _phantom: PhantomData<Store>,
}

#[async_trait]
impl<Store: Strorage + Send + Sync + Clone + 'static> Service<Store> for WebService<Store> {
    fn new(port: u16, lexicon: Lexicon, controller: VotingController<Store>) -> Self {
        let app_state = AxumState {
            controller,
            routes: WEB_ROUTES.clone(),
            lexicon,
        };
        
        let router = make_router(app_state, &WEB_ROUTES);
        let address: SocketAddr = format!("127.0.0.1:{}", port).parse().unwrap();

        Self { address, router, _phantom: PhantomData }
    }

    async fn serve(&self) -> Result<(), anyhow::Error> {
        let listener = tokio::net::TcpListener::bind(&self.address).await.unwrap();
        println!("Service Web en écoute sur http://{}", self.address);
        axum::serve(listener, self.router.clone()).await.unwrap();
        Ok(())
    }
}