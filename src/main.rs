mod error;

use dotenvy::dotenv;
use error::IndexerError;
use std::env;

struct IndexerConfig {
    rpc_url: String,
    commitment_level: String,
    log_level: String,
}

impl IndexerConfig {
    fn from_env() -> Result<Self, IndexerError> {
        dotenv()
            .map_err(|e| IndexerError::Configuration(format!("Failed to load .env file: {}", e)))?;

        let rpc_url = env::var("RPC_URL")
            .map_err(|e| IndexerError::Configuration(format!("RPC_URL missing: {}", e)))?;
        if !rpc_url.starts_with("http") {
            return Err(IndexerError::Configuration(
                "RPC_URL must be a valid HTTP/HTTPS URL".into(),
            ));
        }

        let commitment_level = env::var("COMMITMENT_LEVEL")
            .map_err(|e| IndexerError::Configuration(format!("COMMITMENT_LEVEL missing: {}", e)))?;
        let valid_commitments = ["processed", "confirmed", "finalized"];
        if !valid_commitments.contains(&commitment_level.as_str()) {
            return Err(IndexerError::Configuration(format!(
                "Invalid commitment level: {}. Must be one of: {:?}",
                commitment_level, valid_commitments
            )));
        }

        let log_level = env::var("LOG_LEVEL")
            .map_err(|e| IndexerError::Configuration(format!("LOG_LEVEL missing: {}", e)))?;

        Ok(IndexerConfig {
            rpc_url,
            commitment_level,
            log_level,
        })
    }
}

fn main() {
    println!("Hello, world!");

    let error = IndexerError::Configuration("Configuration error".to_string());
    println!("Error: {}", error);
    // let config = IndexerConfig::from_env().expect("Failed to load configuration");
}
