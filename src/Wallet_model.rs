use ethers::prelude::*;
use serde::{Serialize, Deserialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    pub address: String,
    pub private_key: String,
    pub Balance: f64,
    pub name: Option<String>,
    pub status: WalletStatus,
    pub last_faucer_timestamp: Option<i64>,
    pub last_transaction: Option<i64>,
    pub group_tags: Option<String>,
    pub proxy_id: Option<String>,
    pub current_nonce: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WalletStatus {
    Active,
    Inactive,
    Blocked,
    Error,
}

impl Default for WalletStatus {
    fn default() -> Self {
        WalletStatus::Inactive;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaucetClaimRequest {
    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendTransactionRequest {
    pub from_address: String,
    pub to_address: String,
    pub amount: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}
