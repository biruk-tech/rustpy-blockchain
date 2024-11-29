use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};
use serde_json::{json};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, warn, error};
use crate::message::BlockchainMessage;

pub type SharedSink = Arc<Mutex<futures_util::stream::SplitSink<tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>, Message>>>;
pub type SharedClients = Arc<Mutex<Vec<SharedSink>>>;

/// Start WebSocket server
pub async fn start_websocket_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let listener = TcpListener::bind("127.0.0.1:8081").await?;
    info!("WebSocket server running on ws://127.0.0.1:8081");

    let clients = Arc::new(Mutex::new(Vec::<SharedSink>::new()));

    while let Ok((stream, _)) = listener.accept().await {
        let clients = clients.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, clients).await {
                error!("Error handling connection: {}", e);
            }
        });
    }

    Ok(())
}

async fn handle_connection(
    stream: tokio::net::TcpStream,
    clients: SharedClients,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let ws_stream = accept_async(stream)
        .await
        .map_err(|e| format!("Failed to accept WebSocket connection: {}", e))?;
    info!("New WebSocket connection established");

    let (write, mut read) = ws_stream.split();
    let write = Arc::new(Mutex::new(write));

    {
        let mut clients_lock = clients.lock().await;
        clients_lock.push(write.clone());
    }

    while let Some(msg) = read.next().await {
        match msg {
            Ok(msg) if msg.is_text() => {
                let payload = msg.into_text().unwrap();
                info!("Received message: {}", payload);

                let response = handle_message(&payload).await.unwrap_or_else(|e| {
                    warn!("Error processing message: {}", e);
                    json!({ "type": "error", "message": e }).to_string()
                });

                let mut write_guard = write.lock().await;
                write_guard.send(Message::Text(response)).await.map_err(|e| {
                    format!("Failed to send message: {}", e)
                })?;
            }
            Ok(_) => warn!("Received unsupported WebSocket message"),
            Err(e) => {
                error!("Error receiving message: {}", e);
                break;
            }
        }
    }

    Ok(())
}

async fn handle_message(payload: &str) -> Result<String, String> {
    let blockchain_message: BlockchainMessage = serde_json::from_str(payload)
        .map_err(|e| format!("Invalid message format: {}", e))?;

    if !blockchain_message.is_valid() {
        return Err("Invalid message signature".to_string());
    }

    match blockchain_message.r#type.as_str() {
        "transaction" => {
            info!("Processing transaction: {:?}", blockchain_message.data);

            let response_message = BlockchainMessage::new(
                "response",
                json!({
                    "status": "success",
                    "message": "Transaction processed successfully"
                }),
                "dummy-signature",
            );

            Ok(serde_json::to_string(&response_message).unwrap())
        }
        "ping" => {
            let response_message = BlockchainMessage::new(
                "pong",
                json!({ "message": "pong" }),
                "dummy-signature",
            );

            Ok(serde_json::to_string(&response_message).unwrap())
        }
        "shutdown" => {
            info!("Shutdown request received.");

            let response_message = BlockchainMessage::new(
                "shutdown_ack",
                json!({ "message": "Server shutting down..." }),
                "dummy-signature",
            );

            Ok(serde_json::to_string(&response_message).unwrap())
        }
        _ => {
            warn!("Unknown message type: {}", blockchain_message.r#type);

            let response_message = BlockchainMessage::new(
                "error",
                json!({
                    "message": format!("Unknown message type: {}", blockchain_message.r#type)
                }),
                "dummy-signature",
            );

            Ok(serde_json::to_string(&response_message).unwrap())
        }
    }
}

