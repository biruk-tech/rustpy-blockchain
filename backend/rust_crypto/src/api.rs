use axum::{routing::get, Json, Router};
use base64::{engine::general_purpose, Engine};
use k256::ecdsa::{SigningKey, VerifyingKey};
use rand::thread_rng;
use serde_json::json;
use tracing::info;
// use crate::websocket::WebsocketError;

async fn get_routes() -> String {
    let routes = json!({
        "message": "Rust Server Running.",
        "available_routes": ["/", "/generate_key_pair"]
    });
    routes.to_string()
}

async fn generate_key_pair() -> Json<(String, String)> {
    // Generate signing key (private key) using a secure random number generator
    let signing_key = SigningKey::random(&mut thread_rng());

    // Derive verifying key (public key) from signing key
    let verifying_key = VerifyingKey::from(&signing_key);

    // Encode the private key to Base64
    let private_key = general_purpose::STANDARD.encode(signing_key.to_bytes());

    // Encode the public key to Base64 (uncompressed point representation)
    let public_key =
        general_purpose::STANDARD.encode(verifying_key.to_encoded_point(false).as_bytes());

    Json((public_key, private_key))
}

pub async fn start_server() {
    let app = Router::new()
        .route("/", get(get_routes))
        .route("/generate_key_pair", get(generate_key_pair));

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    info!("Starting HTTP server on port 8080...");
}