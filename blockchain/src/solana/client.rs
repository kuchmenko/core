use std::error::Error;

use crate::{ChainProvider, solana::model::BlockTransactions};
use async_trait::async_trait;
use chrono::Utc;
use ethers::providers::{JsonRpcClient, Http, RetryClientBuilder, RetryClient, RetryClientError, RpcError};
use primitives::{chain::Chain, Transaction, TransactionType, TransactionState, TransactionDirection, asset_id::AssetId};
use reqwest::Url;
use serde_json::json;

use super::model::BlockTransaction;

pub struct SolanaClient {
    client: RetryClient<Http>,
}

const MISSING_SLOT_ERROR: i64 = -32007;
const NOT_AVAILABLE_SLOT_ERROR: i64 = -32004;
const SYSTEM_PROGRAM_ID: &str = "11111111111111111111111111111111";

impl SolanaClient {
    pub fn new(url: String) -> Self {
        let provider = Http::new(Url::parse(url.as_str()).unwrap());
        let client = RetryClientBuilder::default()
            .build(provider, Box::<ethers::providers::HttpRateLimitRetryPolicy>::default());
        
        Self {
            client,
        }
    }

    fn map_transaction(&self, transaction: &BlockTransaction, block_number: i64) -> Option<primitives::Transaction> {
        let account_keys = transaction.transaction.message.account_keys.clone();
        let signatures = transaction.transaction.signatures.clone();

        // system transfer
        if (account_keys.len() == 2 || account_keys.len() == 3) && account_keys.last().unwrap() == SYSTEM_PROGRAM_ID && signatures.len() == 1 && transaction.meta.log_messages.len() == 2  {    
            let chain = self.get_chain();
            let hash = transaction.transaction.signatures.first().unwrap().to_string();
            let from = account_keys.first().unwrap().clone();
            let to = account_keys[account_keys.len() - 2].clone();
            let fee = transaction.meta.fee;
            let value = transaction.meta.pre_balances[0] - transaction.meta.post_balances[0] - fee;  
    
            let transaction = primitives::Transaction{ 
                id: chain.to_string(), 
                hash,
                asset_id: AssetId::from_chain(chain), 
                from, 
                to, 
                contract: None, 
                transaction_type: TransactionType::Transfer, 
                state: TransactionState::Confirmed, 
                block_number: block_number as i32, 
                sequence: 0, 
                fee: fee.to_string(), 
                fee_asset_id: AssetId::from_chain(chain), 
                value: value.to_string(), 
                memo: None,
                direction: TransactionDirection::SelfTransfer, 
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            };
            return Some(transaction);
        }
        return None 
    }
}

#[async_trait]
impl ChainProvider for SolanaClient {

    fn get_chain(&self) -> Chain {
        Chain::Solana
    }

    async fn get_latest_block(&self) -> Result<i64, Box<dyn Error + Send + Sync>> {
        let block: i64 = JsonRpcClient::request(&self.client, "getSlot", ()).await?;
        Ok(block)
    }

    async fn get_transactions(&self, block_number: i64) -> Result<Vec<Transaction>, Box<dyn Error + Send + Sync>> {
        let params = vec![
            json!(block_number),
            json!({
                "encoding": "json",
                "maxSupportedTransactionVersion": 0,
                "transactionDetails": "full",
                "rewards": false
            })
        ];
        let block: Result<BlockTransactions, RetryClientError> = JsonRpcClient::request(&self.client, "getBlock", params).await;
        match block {
            Ok(block) => {
                let transactions = block.transactions
                    .into_iter()
                    .flat_map(|x| self.map_transaction(&x, block_number))
                    .collect::<Vec<primitives::Transaction>>();
                Ok(transactions)
            },
            Err(err) => {
                match err {
                    RetryClientError::ProviderError(err) => {
                        if let Some(json_error) =  err.as_error_response() {
                            if vec![MISSING_SLOT_ERROR, NOT_AVAILABLE_SLOT_ERROR].contains(&json_error.code) {
                                return Ok(vec![])
                            }
                        };
                        return Err(Box::new(err))
                    },
                    _ => {
                        return Err(Box::new(err))
                    }
                }
            }
        } 
    }
}