use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WebsocketError {
    #[error("Error starting websocket server")]
    StartError(#[from] std::io::Error),
}

pub async fn start_websocket_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let listener = TcpListener::bind("127.0.0.1:8081").await.map_err(|e| WebsocketError::StartError(e))?;
    println!("WebSocket server running on ws://127.0.0.1:8081");

    while let Ok((stream, _)) = listener.accept().await {
        let stream = stream;
        tokio::spawn(async move {
            let ws_stream = accept_async(stream).await.unwrap();
            println!("New WebSocket connection: {:?}", ws_stream);
        });
    }
    Ok(())
}
