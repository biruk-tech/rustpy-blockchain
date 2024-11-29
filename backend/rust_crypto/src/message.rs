use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockchainMessage {
    pub r#type: String,
    pub data: serde_json::Value,
    pub signature: String,
}

impl BlockchainMessage {
    pub fn new(msg_type: &str, data: serde_json::Value, signature: &str) -> Self {
        Self {
            r#type: msg_type.to_string(),
            data,
            signature: signature.to_string(),
        }
    }

    pub fn is_valid(&self) -> bool {
        // Add signature validation logic here
        true
    }
}
