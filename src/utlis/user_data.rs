use std::{ error::Error, str::FromStr, time::Duration};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use solana_account_decoder::UiAccountData;
use solana_client::{nonblocking::rpc_client::RpcClient, rpc_client::GetConfirmedSignaturesForAddress2Config, rpc_config::RpcTransactionConfig, rpc_request::TokenAccountsFilter, rpc_response::RpcKeyedAccount};
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey, signature::Signature};
use solana_transaction_status::{EncodedConfirmedTransactionWithStatusMeta, EncodedTransaction, UiInstruction, UiMessage, UiParsedInstruction, UiTransactionEncoding};
use mpl_token_metadata::accounts::Metadata;


#[derive(Debug, Serialize, Deserialize)]
pub struct ParsedData {
    pub info: TokenInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenInfo {
    pub isNative: bool,
    pub mint: String,
    pub owner: String,
    pub state: String,
    pub tokenAmount: TokenAmount,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenAmount {
    pub amount: String,
    pub decimals: u8,
    pub uiAmount: f64,
    pub uiAmountString: String,
}


pub async fn get_user_created_tokens(user: Pubkey, client: RpcClient)->Vec<Metadata> {
    let program_id = Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap();

    let token_accounts:Vec<RpcKeyedAccount> = client
        .get_token_accounts_by_owner(&user, TokenAccountsFilter::ProgramId(program_id)).await
        .unwrap();
    let mut tokens = Vec::new();
    for account in token_accounts {
        let account_data = account.account.data;
        match account_data {
            UiAccountData::Json(parsed_account) => {
                let parsed: Value = parsed_account.parsed.clone();
                if let Ok(token) = serde_json::from_value::<ParsedData>(parsed) {
                    let token_info = token.info;
                    tokens.push(token_info.mint);
                } else {
                    println!("Failed to parse token data");
                }
           
            }
            UiAccountData::Binary(data, _) => {
                println!("Binary data: {}", data);
            }
            UiAccountData::LegacyBinary(_) => todo!(),
            
        }
    }
    let mut user_tokens:Vec<Metadata>= Vec::new();
    for token in tokens {
        let metadata=get_token_metadata(&client, &Pubkey::from_str(&token).unwrap()).await.unwrap();
        let program=find_mint_token_program(&client, &Pubkey::from_str(&token).unwrap()).await.unwrap();
        if !program.is_empty() {
            user_tokens.push(metadata);
        }
    }
    user_tokens

}


pub async fn find_mint_token_program(
    client: &RpcClient,
    mint_address: &Pubkey
) -> Result<String, Box<dyn Error>> {
    let mut total_signatures: Vec<solana_client::rpc_response::RpcConfirmedTransactionStatusWithSignature> =
        Vec::new();

        let mut before: Option<Signature> = None;
    let mut max = 1000;

    loop {
        let signatures_result = client.get_signatures_for_address_with_config(
            mint_address,
            GetConfirmedSignaturesForAddress2Config {
                limit: None,
                commitment: None,
                before,
                until: None,
            }
        ).await;

        if !signatures_result.is_ok() {
            println!("signatures_result {:?}", signatures_result);
            break;
        }
        // println!("signatures_result {:?}", signatures_result.is_ok());
        let _signatures_result = signatures_result.unwrap().clone();

        before = Some(Signature::from_str(&_signatures_result.last().unwrap().signature).unwrap());
        total_signatures.extend(_signatures_result.clone());
        if total_signatures.len() == max {
            max += 1000;
        } else {
            break;
        }
    }

    let signature_info = &total_signatures.last().unwrap();

    let signature = Signature::from_str(&signature_info.signature).unwrap();

    let tx_option: EncodedConfirmedTransactionWithStatusMeta = client.get_transaction_with_config(
        &signature,
        RpcTransactionConfig {
            encoding: Some(UiTransactionEncoding::JsonParsed),
            commitment: Some(CommitmentConfig::finalized()),
            max_supported_transaction_version: Some(0),
        }
    ).await?;

    let transaction: &solana_transaction_status::EncodedTransaction = &tx_option.transaction.transaction;
    match transaction {
        EncodedTransaction::Json(parsed_account) => {match &parsed_account.message {
            UiMessage::Parsed(message) => {
                for instruction in &message.instructions {
                    
                    match instruction {
                        UiInstruction::Parsed(instruction)=>{
                            match  instruction{
                                UiParsedInstruction::PartiallyDecoded(instruction) => {
                                    if instruction.program_id.to_string() =="MoonCVVNZFSYkqNXP6bxHLPL6QQJiMagDL3qcqUQTrG" {
                                        return Ok(instruction.program_id.to_string())
                                    }else {
                                        continue;
                                    
                                    }
                                }
                                UiParsedInstruction::Parsed(_) => {continue},
                            }
                        }
                        UiInstruction::Compiled(_) => todo!(),
                    }
                }
            }
            UiMessage::Raw(_) => todo!(),
            
        }}
        EncodedTransaction::LegacyBinary(_) => todo!(),
        EncodedTransaction::Binary(_, _) => todo!(),
        EncodedTransaction::Accounts(_) => todo!(), }
    Ok("".to_owned())


}


pub async fn get_token_metadata(
    client: &RpcClient,
    mint_address: &Pubkey
) -> Result<Metadata, Box<dyn Error>> {
    let pump_program_id = Pubkey::from_str("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s").expect(
        "Failed to parse pubkey"
    );
    let (metadata_key, _) = Pubkey::find_program_address(
        &[b"metadata", &pump_program_id.as_ref(), &mint_address.as_ref()],
        &pump_program_id
    );

    let account_data = match client.get_account_data(&metadata_key).await {
        Ok(data) => data,
        Err(e) => {
            return Err(Box::new(e));
        }
    };
    let metadata: Metadata = Metadata::safe_deserialize(&mut &account_data).map_err(|e|
        format!("Failed to deserialize metadata: {:?}", e)
    )?;

    Ok(metadata)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenMetadata {
    pub name: String,
    pub symbol: String,
    pub description: String,
    pub image: String,
}

pub async fn get_pump_token_metadata(uri: &str) -> anyhow::Result<TokenMetadata> {
 println!("uri {:?}", uri);
 
    let client = Client::builder()
        .timeout(Duration::from_secs(20)) // Set the timeout duration here
        .build()?;

    let resp = client.get(uri).send().await?;
    let resp_str = resp.text().await?;

    let body: TokenMetadata = serde_json::from_str(&resp_str)?;
   
    Ok(body)
}