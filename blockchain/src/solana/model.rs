
use typeshare::typeshare;
use serde::{Serialize, Deserialize};

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub blockhash: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    //pub compute_units_consumed: u64,
    //pub err: Option<String>,
    pub fee: u64,
    //pub inner_instructions: Vec<String>,
    //pub loaded_addresses: LoadedAddresses,
    pub log_messages: Vec<String>,
    pub post_balances: Vec<u64>,
    //pub post_token_balances: Vec<String>,
    pub pre_balances: Vec<u64>,
    //pub pre_token_balances: Vec<String>,
    //pub rewards: Option<String>,
    //pub status: Status,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub ok: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub account_keys: Vec<String>,
    //pub instructions: Vec<Instruction>,
    //pub recent_blockhash: String,
}

// #[derive(Debug, Deserialize, Serialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Instruction {
//     pub accounts: Vec<u64>,
//     pub data: String,
//     pub program_id_index: u64,
// }

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub message: Message,
    pub signatures: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockTransaction {
    pub meta: Meta,
    pub transaction: Transaction,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockTransactions {
    pub transactions: Vec<BlockTransaction>
}