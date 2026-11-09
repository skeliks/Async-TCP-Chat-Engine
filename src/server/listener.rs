use crate::server::connection::handle_connection;
use std::io;
use tokio::net::TcpListener;

/// Starts the TCP server and listens for incoming connections.
pub async fn run() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (stream, _) = listener.accept().await?;

        handle_connection(stream).await?;
    }
}
