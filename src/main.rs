pub mod ws_client;
pub mod event;

use tokio_tungstenite::tungstenite::protocol::Message;
use futures_util::StreamExt;
use serde_json::json;
use ws_client::subscribe_experimental;
const WS_URL: &str = "wss://mulberry.rpcpool.com/4bcd1601-da1f-401d-9b5d-5e1c84208424/whirligig";

#[tokio::main]
async fn main() {
    let method = "transactionSubscribe";
    let params =
        json!([
        {
            "failed": false,
            "accounts": {
                "include": ["MoonCVVNZFSYkqNXP6bxHLPL6QQJiMagDL3qcqUQTrG"]
            }
        },
        {
            "commitment": "processed",
            "encoding": "jsonParsed",
            "transactionDetails": "full",
            "maxSupportedTransactionVersion": 0
        }
    ]);

    let (response, mut stream) = subscribe_experimental(WS_URL, method, params).await.unwrap();
    println!("Subscribe response: {:?}", response);

    while let Some(message) = stream.next().await {
        match message {
            Ok(Message::Text(data)) => {
                let message_obj: serde_json::Value = serde_json::from_str(&data).unwrap();
      
                event::parse_pump_event(message_obj).await;
            }

            Err(e) => eprintln!("Error: {}", e),
            _ => {}
        }
    }
    stream.close(None).await.unwrap();
    println!("WebSocket connection closed");
}
