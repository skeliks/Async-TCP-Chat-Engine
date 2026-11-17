//! Handles individual client TCP connections.
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

/// Handles an individual client TCP connection.
///
/// Reads data from the client and echoes it back
/// until the connection is closed or an I/O error occurs.
///
/// # Errors
///
/// Returns an error if reading from or writing to the TCP stream fails.
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
