use futures_util::{SinkExt, StreamExt};
use std::env;
use tokio::time::{timeout, Duration};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

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
    let url = env::var("WS_HOST").unwrap_or("ws://127.0.0.1:53939".to_string());

    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");

    let (mut write, mut read) = ws_stream.split();
    //
    write
        .send(Message::Text("Hello WebSocket Server#️⃣".into()))
        .await?;

    //Send message to websocket server
    while let Some(message) = read.next().await {
        let message = message.expect("Failed to receive message");
        match message {
            Message::Text(text) => {
                println!("Received a text message: {}", text);
            }
            Message::Binary(bin) => {
                println!("Received a binary message: {:?}", bin);
            }
            Message::Ping(_) => todo!(),
            Message::Pong(_) => todo!(),
            Message::Frame(_) => todo!(),
            Message::Close(_) => {
                println!("Received a close message");
                // サーバーからCloseメッセージを受信したら、クローズ処理を行う
                break;
            }
        }
    }

    // Send a Close message to initiate the closing handshake
    write
        .send(Message::Close(None))
        .await
        .expect("Failed to send close message");

    // Wait up to 5 seconds to receive a Close message from the server
    if let Ok(_) = timeout(Duration::from_secs(5), write.close()).await {
        println!("Connection closed gracefully.");
    } else {
        println!("Timeout waiting for server to close the connection.");
    }

    Ok(())
}
