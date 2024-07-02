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

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction2 {
    message: Option<Message>,
    signatures: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    version: Option<u64>,
    meta: Option<Meta>,
    transaction: Option<Transaction2>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    accountKeys: Vec<AccountKey>,
    addressTableLookups: Vec<AddressTableLookup>,
    instructions: Vec<Instruction>,
    recentBlockhash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountKey {
    pubkey: String,
    signer: bool,
    source: String,
    writable: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddressTableLookup {
    // Define the fields here if any
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Instruction {
    accounts: Vec<String>,
    data: String,
    programId: String,
    stackHeight: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct InnerInstruction {
    index: u64,
    instructions: Vec<ParsedInstruction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParsedInstruction {
    parsed: Option<Parsed>,
    program: Option<String>,
    programId: String,
    stackHeight: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parsed {
    info: Info,
    #[serde(rename = "type")]
    type_: String,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenAmount {
    amount: String,
    decimals: u64,
    uiAmount: f64,
    uiAmountString: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenBalance {
    accountIndex: u64,
    mint: String,
    owner: String,
    programId: String,
    uiTokenAmount: TokenAmount,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    Ok: Option<serde_json::Value>,
}

#[derive(Debug, Clone)]
pub enum PumpEvent {
    SellEvent(SellEvent),
    BuyEvent(BuyEvent),
    CreateEvent(CreateEvent),
}

#[derive(Debug, Clone)]
pub struct CreateEvent {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub user: Pubkey,
}

impl CreateEvent {
    pub fn from_hex(hex_data: &str, accounts: Vec<String>) -> anyhow::Result<Self> {
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

        let mint_data: [u8; 32] =
            bytes[uri_offset + uri_len..uri_offset + uri_len + 32].try_into()?;
        let mint = Pubkey::from(mint_data);
        let bonding_curve_data: [u8; 32] =
            bytes[uri_offset + uri_len + 32..uri_offset + uri_len + 64].try_into()?;
        let bonding_curve = Pubkey::from(bonding_curve_data);
        let user_data: [u8; 32] = bytes[uri_offset + uri_len + 64..uri_offset + uri_len + 96]
            .try_into()
            .unwrap();
        let user = Pubkey::from(user_data);

        Ok(CreateEvent {
            name,
            symbol,
            uri,
            mint,
            bonding_curve,
            user,
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct SellEvent {
    pub amount: u64,
    pub sender: Pubkey,
    pub curve_account: Pubkey,
    pub mint: Pubkey,
}

impl SellEvent {
    pub fn from_hex(hex_data: &str, accounts: Vec<String>) -> anyhow::Result<Self> {
        let bytes = hex::decode(hex_data).expect("Decoding failed");
        let amount = u64::from_le_bytes(bytes[8..16].try_into()?);

        let sender = accounts[0].parse().unwrap();
        let curve_account = accounts[3].parse().unwrap();
        let mint = accounts[9].parse().unwrap();
        Ok(SellEvent {
            amount,
            sender,
            curve_account,
            mint,
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct BuyEvent {
    pub amount: u64,
    pub sender: Pubkey,
    pub curve_account: Pubkey,
    pub mint: Pubkey,
}

impl BuyEvent {
    pub fn from_hex(hex_data: &str, accounts: Vec<String>) -> anyhow::Result<Self> {
        let bytes = hex::decode(hex_data).expect("Decoding failed");

        let amount = u64::from_le_bytes(bytes[8..16].try_into()?);

        let sender = accounts[0].parse().unwrap();
        let curve_account = accounts[3].parse().unwrap();
        let mint = accounts[9].parse().unwrap();
        Ok(BuyEvent {
            amount,
            sender,
            curve_account,
            mint,
        })
    }
}
pub async fn parse_pump_event(message_obj: serde_json::Value) {
    let message_obj: TransactionNotification = match
        serde_json::from_value::<TransactionNotification>(message_obj.clone())
    {
        Ok(message) => message,
        Err(e) => {
            println!("Error parsing message: {:?}", e);
            println!("Message: {:#?}", message_obj);
            return;
        }
    };

    let data: String = message_obj.params.result.value.transaction
        .unwrap()
        .transaction.unwrap()
        .message.unwrap()
        .instructions[1].data.clone();

    let decoded_bytes = bs58::decode(data.clone()).into_vec().unwrap();

    let hex_str: String = decoded_bytes
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();

    println!("Hex encoded data: {}", hex_str);
    println!("data: {:#?}", data);

    let disc_hex = &hex_str[0..16];

    println!("disc_hex: {:#?}", disc_hex);

    match disc_hex {
        "33e685a4017f83ad" => {
            println!("Sell Event {:?}", data);
        }
        "66063d1201daebea" => {
            println!("Buy Event {:?}", data);
        }
        "032ca4b87b0df5b3" => {
            println!("Create Event {:?}", data);
        }
        _ => {
            println!("Unknown Event {:?}", data);
        }
    }
}
