use futures_util::{SinkExt, StreamExt};
use std::env;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read environment variable `RUST_ENV`
    let rust_env = env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string());

    // Set the .env file path according to the environment
    let env_file = match rust_env.as_str() {
        "production" => ".env.prod",
        _ => ".env.dev",
    };

    // Load environment variable file using dotenv
    dotenv::from_filename(env_file).ok();
    // Start TCP listener
    let listen_addr = env::var("LISTEN_ADDR").unwrap_or("127.0.0.1:53939".to_string());
    let listener = TcpListener::bind(listen_addr).await?;
    let addr = listener.local_addr()?;

    println!("Listening TCP on: {}", addr);
    while let Ok((stream, _)) = listener.accept().await {
        //TODO: Connection restriction and management (authentication processing added)
        tokio::spawn(handle_connection(stream));
    }

    Ok(())
}

async fn handle_connection(stream: tokio::net::TcpStream) {
    let ws_stream = accept_async(stream)
        .await
        .expect("Error during WebSocket handshake occurred");
    println!(
        "New WebSocket connection: {}",
        ws_stream.get_ref().peer_addr().unwrap()
    );

    let (mut write, mut read) = ws_stream.split();

    // Receive messages from clients and reply directly to them
    while let Some(message) = read.next().await {
        let message = message.expect("Failed to read message from client");
        match message {
            Message::Text(text) => {
                println!("Received a text message: {}", text);
                write
                    .send(Message::Text(text))
                    .await
                    .expect("Failed to send message to client");
            }
            Message::Binary(bin) => {
                println!("Received a binary message: {:?}", bin);
            }
            Message::Ping(_) => todo!(),
            Message::Pong(_) => todo!(),
            Message::Frame(_) => todo!(),
            Message::Close(_) => {
                println!("Client requests closing the connection.");
                write
                    .send(Message::Close(None))
                    .await
                    .expect("Failed to send close message");
                break;
            }
        }
    }
}
