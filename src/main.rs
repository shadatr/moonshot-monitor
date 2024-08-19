pub mod ws_client;
pub mod event;
pub mod new_tokens;
pub mod utlis;
pub mod consts;

use consts::{RPC_URL, WS_URL};
use event::MoonEvent;
use new_tokens::new_tokens_prog;
use solana_client::nonblocking::rpc_client::RpcClient;
use tokio_tungstenite::tungstenite::protocol::Message;
use futures_util::StreamExt;
use serde_json::json;
use utlis::user_data::{get_pump_token_metadata, get_user_created_tokens};
use ws_client::subscribe_experimental;

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

    let (_, mut stream) = subscribe_experimental(WS_URL, method, params).await.unwrap();
    
    
        while let Some(message) = stream.next().await {
        match message {
            Ok(Message::Text(data)) => {
                let message_obj: serde_json::Value = serde_json::from_str(&data).unwrap();
                println!("true");
                match event::parse_pump_event(message_obj.clone()).await {
                    Some(event) =>
                    match event {
                        MoonEvent::BuyEvent(_) => {
                        }
                        MoonEvent::SellEvent(_) => {
                        }
                        MoonEvent::CreateEvent(create_event) => {
                            println!("{:?}", create_event);
                            let rpc_client: RpcClient = RpcClient::new(RPC_URL.to_string());
                            let token_data=get_pump_token_metadata(&create_event.uri).await.unwrap();
                                let token_accounts = get_user_created_tokens(create_event.sender, rpc_client).await;
                                // let token_accounts= Vec::new();

                                println!("token_accounts {:?}", token_accounts);
                                println!("token_data {:?}", token_data);
                                let task1 = tokio::spawn(new_tokens_prog(create_event.clone(), token_data.clone(),token_accounts.clone()));

                                let _ = tokio::try_join!(task1, 
                                );
                            }
                        }
                    None => {
                        continue;
                    }
                };
            }

            Err(e) => eprintln!("Error: {}", e),
            _ => {}
        }
    }
    stream.close(None).await.unwrap();
    println!("WebSocket connection closed");
}
