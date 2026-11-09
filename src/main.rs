use async_tcp_chat_engine::server::listener;
use tokio::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    listener::run().await
}
