mod api;
mod websocket;
use tokio::task;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    // Running WebSocket server and HTTP API concurrently
    let websocket_server = task::spawn(websocket::start_websocket_server());
    let http_api_server = task::spawn(api::start_server());

    // Wait for both servers to complete
    match tokio::try_join!(websocket_server, http_api_server) {
        Ok(_) => Ok(()),
        Err(err) => Err(std::io::Error::new(std::io::ErrorKind::Other, err)),
    }
}
