use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};
use serde_json::json;
use std::sync::Arc;
use tracing::{info, warn };
use crate::message::BlockchainMessage;
use k256::ecdsa::{SigningKey, VerifyingKey};
use rand_core::OsRng;

async fn handle_message(
    payload: &str,
    signing_key: &SigningKey,
    verifying_key: &VerifyingKey,
) -> Result<Option<String>, String> {
    // Deserialize the message
    let blockchain_message: BlockchainMessage = serde_json::from_str(payload)
        .map_err(|e| format!("Invalid message format: {}", e))?;

    // Verify the incoming message's signature
    if !blockchain_message.verify_signature(verifying_key) {
        return Err("Invalid message signature".to_string());
    }

    match blockchain_message.r#type.as_str() {
        "transaction" => {
            info!("Processing transaction: {:?}", blockchain_message.data);

            let mut response_message = BlockchainMessage::new(
                "response",
                json!({
                    "status": "success",
                    "message": "Transaction processed successfully"
                }),
            );

            // Sign the response
            response_message.sign(signing_key);

            Ok(serde_json::to_string(&response_message).ok())
        }
        "ping" => {
            let mut response_message = BlockchainMessage::new(
                "pong",
                json!({
                    "message": "pong"
                }),
            );

            // Sign the response
            response_message.sign(signing_key);

            Ok(serde_json::to_string(&response_message).ok())
        }
        "shutdown" => {
            info!("Shutdown request received.");

            let mut response_message = BlockchainMessage::new(
                "shutdown_ack",
                json!({
                    "message": "Server shutting down..."
                }),
            );

            // Sign the response
            response_message.sign(signing_key);

            Ok(serde_json::to_string(&response_message).ok())
        }
        _ => {
            warn!("Unknown message type: {}", blockchain_message.r#type);

            let mut response_message = BlockchainMessage::new(
                "error",
                json!({
                    "message": format!("Unknown message type: {}", blockchain_message.r#type)
                }),
            );

            // Sign the response
            response_message.sign(signing_key);

            Ok(serde_json::to_string(&response_message).ok())
        }
    }
}

pub async fn start_websocket_server() -> Result<(), tokio_tungstenite::tungstenite::Error> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8081").await?;
    tracing::info!("WebSocket server running on ws://127.0.0.1:8081");
    let signing_key = SigningKey::random(&mut OsRng);
    let _verifying_key = signing_key.verifying_key(); 
    
    let signing_key = Arc::new(SigningKey::random(&mut OsRng)); // Wrap `signing_key` directly in `Arc`
    let verifying_key = Arc::new(VerifyingKey::from(signing_key.as_ref())); // Create an owned `VerifyingKey`

    while let Ok((stream, _)) = listener.accept().await {
        let signing_key = Arc::clone(&signing_key);
        let verifying_key = Arc::clone(&verifying_key);

        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, &signing_key, &verifying_key).await {
                tracing::error!("Connection error: {}", e);
            }
        });
    }

    Ok(())
}

async fn handle_connection(
    stream: tokio::net::TcpStream,
    signing_key: &SigningKey,
    verifying_key: &VerifyingKey,
) -> Result<(), tokio_tungstenite::tungstenite::Error> {
    let ws_stream = accept_async(stream).await?;
    info!("New WebSocket connection established");

    let (mut write, mut read) = ws_stream.split();

    while let Some(msg) = read.next().await {
        if let Ok(msg) = msg {
            if msg.is_text() {
                let payload = msg.into_text().unwrap();

                match handle_message(&payload, signing_key, verifying_key).await {
                    Ok(Some(response)) => {
                        write.send(tokio_tungstenite::tungstenite::protocol::Message::Text(response))
                            .await
                            .unwrap();
                    }
                    Ok(None) => (), // No response needed
                    Err(e) => {
                        warn!("Failed to process message: {}", e);
                        write.send(tokio_tungstenite::tungstenite::protocol::Message::Text(
                            "Error processing message".to_string(),
                        ))
                        .await
                        .unwrap();
                    }
                }
            }
        }
    }

    Ok(())
}
