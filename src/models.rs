use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenCreation {
    pub mint_address: String,
    pub creator: String,
    pub initial_supply: u64,
    pub decimals: u8,
    pub signature: String,
    pub slot: u64,
    pub timestamp: Option<i64>,
}

impl TokenCreation {
    pub fn new(
        mint_address: String,
        creator: String,
        initial_supply: u64,
        decimals: u8,
        signature: String,
        slot: u64,
        timestamp: Option<i64>,
    ) -> Self {
        Self {
            mint_address,
            creator,
            initial_supply,
            decimals,
            signature,
            slot,
            timestamp,
        }
    }
}
