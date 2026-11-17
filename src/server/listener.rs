use crate::server::connection::handle_connection;
use std::{
    io,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    thread,
};
use tokio::net::TcpListener;

/// Starts the TCP server, accepts incoming connections,
/// and handles server commands.
pub async fn run() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    let active_connections = Arc::new(AtomicUsize::new(0));
    let connection_counter = Arc::clone(&active_connections);

    thread::spawn(move || {
        loop {
            let mut command = String::new();

            if io::stdin().read_line(&mut command).unwrap_or(0) == 0 {
                break;
            }

            let command = command.trim();

            match command {
                "connections_count" => {
                    println!(
                        "Current number of connections: {}",
                        connection_counter.load(Ordering::Relaxed)
                    );
                }
                _ => println!("Unknown command: {command}"),
            }
        }
    });

    loop {
        let (stream, addr) = listener.accept().await?;
        let active_connections = Arc::clone(&active_connections);

        active_connections.fetch_add(1, Ordering::Relaxed);
        println!("Client connected: {addr}");

        tokio::spawn(async move {
            match handle_connection(stream).await {
                Ok(_) => println!("Client disconnected: {addr}"),
                Err(err) => eprintln!("Client at {addr} disconnected with error: {err}"),
            }

            active_connections.fetch_sub(1, Ordering::Relaxed);
        });
    }
}
