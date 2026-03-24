use axum::{routing::{get, post}, Router};
use crate::storage::Strorage;
use super::{AxumState, web_routes::WebRoutes, html_handlers};
use super::json::v1::v1_handler;

pub fn make_router<Store: Strorage + Clone + Send + Sync + 'static>(
    app_state: AxumState<Store>,
    routes: &WebRoutes,
) -> Router {
    let v1_routes = Router::new()
        .route(routes.vote, post(v1_handler::vote))
        .route(routes.results, get(v1_handler::get_results));

    let json_routes = Router::new().nest(routes.v1, v1_routes);

    Router::new()
        .route(routes.index, get(html_handlers::get_index))
        .route(routes.vote, post(html_handlers::vote))
        .route(routes.results, get(html_handlers::get_results))
        .nest(routes.json, json_routes)
        .with_state(app_state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, header, StatusCode},
    };
    use http_body_util::BodyExt; // Pour pouvoir extraire les bytes de la réponse
    use tower::ServiceExt; // Fournit la méthode `oneshot` pour le test
    use serde_json::Value;

    use crate::domain::{Candidate, Votingmachine};
    use crate::storages::memory::MemoryStore;
    use crate::use_case::VotingController;
    use crate::interfaces::lexicons::french::FRENCH;
    use crate::interfaces::web_interfaces::web_routes::WEB_ROUTES;

    async fn setup_app() -> Router {
        // Initialisation de la machine avec un candidat
        let machine = Votingmachine::new(vec![Candidate("c1".to_string())]);
        let store = MemoryStore::new(machine).await.unwrap();
        let controller = VotingController::new(store);
        
        let app_state = AxumState {
            controller,
            routes: WEB_ROUTES.clone(),
            lexicon: FRENCH.clone(),
        };
        
        make_router(app_state, &WEB_ROUTES)
    }

    #[tokio::test]
    async fn test_html_interface() {
        let app = setup_app().await;

        // 1. Voter via l'interface HTML
        let request = Request::builder()
            .method("POST")
            .uri("/vote")
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(Body::from("voter=alice&candidat=c1"))
            .unwrap();
        
        let response = app.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        // 2. Vérifier les résultats HTML
        let request = Request::builder().uri("/results").body(Body::empty()).unwrap();
        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let html = String::from_utf8(body.to_vec()).unwrap();
        assert!(html.contains("c1")); // On vérifie que c1 est bien dans l'affichage HTML
    }

    #[tokio::test]
    async fn test_json_interface() {
        let app = setup_app().await;

        // 1. Voter via l'interface JSON
        let request = Request::builder()
            .method("POST")
            .uri("/json/v1/vote")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(r#"{"voter":"bob", "candidate":"c1"}"#))
            .unwrap();

        let response = app.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let json: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["AcceptedVote"][0], "bob");
        assert_eq!(json["AcceptedVote"][1], "c1");

        // 2. Vérifier les résultats JSON
        let request = Request::builder().uri("/json/v1/results").body(Body::empty()).unwrap();
        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let json: Value = serde_json::from_slice(&body).unwrap();
        
        assert!(json["voters"].as_array().unwrap().contains(&serde_json::json!("bob")));
        assert_eq!(json["scores"]["c1"], 1);
    }
}
