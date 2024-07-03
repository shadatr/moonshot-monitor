use std::str::FromStr;

use solana_sdk::pubkey::Pubkey;

use serde::{ Deserialize, Serialize };

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionNotification {
    jsonrpc: String,
    method: String,
    params: Params,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Params {
    result: ResultField,
    subscription: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultField {
    context: Context,
    value: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Context {
    slot: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Value {
    slot: u64,
    transaction: Option<Transaction>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction2 {
    message: Option<Message>,
    signatures: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    version: Option<u64>,
    meta: Option<Meta>,
    transaction: Option<Transaction2>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    accountKeys: Vec<AccountKey>,
    addressTableLookups: Vec<AddressTableLookup>,
    instructions: Vec<Instruction>,
    recentBlockhash: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountKey {
    pubkey: String,
    signer: bool,
    source: String,
    writable: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AddressTableLookup {
    // Define the fields here if any
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Instruction {
    accounts: Vec<String>,
    data: String,
    programId: String,
    stackHeight: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Meta {
    computeUnitsConsumed: u64,
    err: Option<serde_json::Value>,
    fee: u64,
    innerInstructions: Vec<InnerInstruction>,
    logMessages: Vec<String>,
    postBalances: Vec<u64>,
    postTokenBalances: Vec<TokenBalance>,
    preBalances: Vec<u64>,
    preTokenBalances: Vec<TokenBalance>,
    rewards: Option<serde_json::Value>,
    status: Status,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InnerInstruction {
    index: u64,
    instructions: Vec<ParsedInstruction>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ParsedInstruction {
    parsed: Option<Parsed>,
    program: Option<String>,
    programId: String,
    stackHeight: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Parsed {
    info: Info,
    #[serde(rename = "type")]
    type_: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Info {
    account: Option<String>,
    mint: Option<String>,
    source: Option<String>,
    systemProgram: Option<String>,
    tokenProgram: Option<String>,
    wallet: Option<String>,
    extensionTypes: Option<Vec<String>>,
    lamports: Option<u64>,
    newAccount: Option<String>,
    owner: Option<String>,
    space: Option<u64>,
    authority: Option<String>,
    destination: Option<String>,
    tokenAmount: Option<TokenAmount>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenAmount {
    amount: String,
    decimals: u64,
    uiAmount: f64,
    uiAmountString: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenBalance {
    accountIndex: u64,
    mint: String,
    owner: String,
    programId: String,
    uiTokenAmount: TokenAmount,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Status {
    Ok: Option<serde_json::Value>,
}

#[derive(Debug, Clone)]
pub enum MoonEvent {
    SellEvent(SellEvent),
    BuyEvent(BuyEvent),
    CreateEvent(CreateEvent),
}

#[derive(Debug, Clone)]
pub struct CreateEvent {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub sender: Pubkey,
    pub curve_account: Pubkey,
    pub mint: Pubkey,
    pub buy_event: Option<BuyEvent>,
}

impl CreateEvent {
    pub fn from_hex(
        hex_data: &str,
        accounts: Vec<String>,
        hex_data_buy_dev: &str
    ) -> anyhow::Result<Self> {
        let bytes = hex::decode(hex_data)?;

        // Extract data according to the given structure
        let name_len = u32::from_le_bytes(bytes[8..12].try_into()?) as usize;
        let name = String::from_utf8(bytes[12..12 + name_len].to_vec())?.to_string();

        let symbol_offset = 12 + name_len + 4; // including the 4 bytes for the length
        let symbol_len = u32::from_le_bytes(
            bytes[symbol_offset - 4..symbol_offset].try_into()?
        ) as usize;
        let symbol = String::from_utf8(
            bytes[symbol_offset..symbol_offset + symbol_len].to_vec()
        )?.to_string();

        let uri_offset = symbol_offset + symbol_len + 4; // including the 4 bytes for the length
        let uri_len = u32::from_le_bytes(bytes[uri_offset - 4..uri_offset].try_into()?) as usize;
        let uri = String::from_utf8(bytes[uri_offset..uri_offset + uri_len].to_vec())?.to_string();

        let sender = &accounts[0];
        let curve_account = &accounts[2];
        let mint = &accounts[3];
        if hex_data_buy_dev.is_empty() {
            return Ok(CreateEvent {
                name,
                symbol,
                uri,
                sender: Pubkey::from_str(&sender).unwrap(),
                curve_account: Pubkey::from_str(&curve_account).unwrap(),
                mint: Pubkey::from_str(&mint).unwrap(),
                buy_event: None,
            });
        }
        let buy_event = BuyEvent::from_hex(
            hex_data_buy_dev,
            accounts.clone()
        ).unwrap();

        Ok(CreateEvent {
            name,
            symbol,
            uri,
            sender: Pubkey::from_str(&sender).unwrap(),
            curve_account: Pubkey::from_str(&curve_account).unwrap(),
            mint: Pubkey::from_str(&mint).unwrap(),
            buy_event: Some(buy_event),
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct SellEvent {
    pub amount: u64,
    pub collateral_amount: u64,
    pub slippage_bps: u64,
    pub sender: Pubkey,
    pub curve_account: Pubkey,
    pub mint: Pubkey,
}

impl SellEvent {
    pub fn from_hex(hex_data: &str, accounts: Vec<String>) -> anyhow::Result<Self> {
        let bytes = hex::decode(hex_data).expect("Decoding failed");
        let amount = u64::from_le_bytes(bytes[8..16].try_into()?);
        let collateral_amount = u64::from_le_bytes(bytes[16..24].try_into()?);
        let slippage_bps = u64::from_le_bytes(bytes[24..32].try_into()?);
        let sender = &accounts[0];
        let curve_account = &accounts[3];
        let mint = &accounts[7];

        Ok(SellEvent {
            amount,
            collateral_amount,
            slippage_bps: slippage_bps,
            sender: Pubkey::from_str(&sender).unwrap(),
            curve_account: Pubkey::from_str(&curve_account).unwrap(),
            mint: Pubkey::from_str(&mint).unwrap(),
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct BuyEvent {
    pub amount: u64,
    pub collateral_amount: u64,
    pub slippage_bps: u64,
    pub sender: Pubkey,
    pub curve_account: Pubkey,
    pub mint: Pubkey,
}

impl BuyEvent {
    pub fn from_hex(hex_data: &str, accounts: Vec<String>) -> anyhow::Result<Self> {
        let bytes = hex::decode(hex_data).expect("Decoding failed");
        let amount = u64::from_le_bytes(bytes[8..16].try_into()?);
        let collateral_amount = u64::from_le_bytes(bytes[16..24].try_into()?);
        let slippage_bps = u64::from_le_bytes(bytes[24..32].try_into()?);
        let sender = &accounts[0];
        let curve_account = &accounts[3];
        let mint = &accounts[7];

        Ok(BuyEvent {
            amount,
            collateral_amount,
            slippage_bps: slippage_bps,
            sender: Pubkey::from_str(&sender).unwrap(),
            curve_account: Pubkey::from_str(&curve_account).unwrap(),
            mint: Pubkey::from_str(&mint).unwrap(),
        })
    }
}
pub async fn parse_pump_event(message_obj: serde_json::Value) -> Option<MoonEvent> {
    let message_obj: TransactionNotification = serde_json
        ::from_value::<TransactionNotification>(message_obj.clone())
        .unwrap();

    let data: String = message_obj.params.result.value.transaction
        .clone()
        .unwrap()
        .transaction.unwrap()
        .message.unwrap()
        .instructions[1].data.clone();

    let decoded_bytes = bs58::decode(data.clone()).into_vec().unwrap();
    let mut decoded_bytes_dev_buy: Vec<u8> = Vec::new();

    if
        message_obj.params.result.value.transaction
            .clone()
            .unwrap()
            .transaction.unwrap()
            .message.unwrap()
            .instructions.len() > 2
    {
        let data_dev_buy: String = message_obj.params.result.value.transaction
            .clone()
            .unwrap()
            .transaction.unwrap()
            .message.unwrap()
            .instructions[2].data.clone();

        decoded_bytes_dev_buy = bs58::decode(data_dev_buy.clone()).into_vec().unwrap();
    }

    let hex_str: String = decoded_bytes
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();

    let disc_hex = &hex_str[0..16];
    let accounts: Vec<String> = message_obj.params.result.value.transaction
        .clone()
        .unwrap()
        .transaction.unwrap()
        .message.unwrap()
        .instructions[1].accounts.clone();

    match disc_hex {
        "33e685a4017f83ad" => {
            let sell_event: SellEvent = match
                SellEvent::from_hex(&hex::encode(decoded_bytes.as_slice()), accounts.clone())
            {
                Ok(buy_event) => buy_event,
                Err(_) => {
                    return None;
                }
            };

            Some(MoonEvent::SellEvent(sell_event))
        }
        "66063d1201daebea" => {
            let buy_event: BuyEvent = match
                BuyEvent::from_hex(&hex::encode(decoded_bytes.as_slice()), accounts.clone())
            {
                Ok(buy_event) => buy_event,
                Err(_) => {
                    return None;
                }
            };
            Some(MoonEvent::BuyEvent(buy_event))
        }
        "032ca4b87b0df5b3" => {
            let create_event: CreateEvent = match
                CreateEvent::from_hex(
                    &hex::encode(decoded_bytes.as_slice()),
                    accounts.clone(),
                    &hex::encode(decoded_bytes_dev_buy.as_slice())
                )
            {
                Ok(create_event) => { create_event }
                Err(_) => {
                    return None;
                }
            };
            Some(MoonEvent::CreateEvent(create_event))
        }
        _ => {
            println!("Unknown Event {:?}", data);
            None
        }
    }
}
