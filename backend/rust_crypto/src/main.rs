mod api;
mod websocket;
mod message;
use tokio::task;
use tracing_subscriber;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    tracing_subscriber::fmt::init();

    // Run WebSocket and HTTP servers concurrently
    let websocket_server = task::spawn(websocket::start_websocket_server());
    let http_api_server = task::spawn(api::start_server());

    // Wait for both servers to complete
    match tokio::try_join!(websocket_server, http_api_server) {
        Ok(_) => Ok(()),
        Err(err) => Err(std::io::Error::new(std::io::ErrorKind::Other, err)),
    }
}
