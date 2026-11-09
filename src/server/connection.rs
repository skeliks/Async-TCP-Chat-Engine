//! Handles individual client TCP connections.
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

/// Handles an individual TCP connection.
///
/// Reads incoming bytes and echoes them back to the client
/// until the connection is closed.
pub async fn handle_connection(socket: TcpStream) -> io::Result<()> {
    let (mut reader, mut writer) = io::split(socket);
    let mut buffer = vec![0u8; 1024];

    loop {
        let bytes_read = reader.read(&mut buffer).await?;

        if bytes_read == 0 {
            break;
        }

        writer.write_all(&buffer[..bytes_read]).await?;
    }

    Ok(())
}
