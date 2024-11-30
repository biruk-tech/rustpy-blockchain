use k256::ecdsa::{SigningKey, VerifyingKey, Signature};
use k256::ecdsa::signature::{Signer, Verifier};
use serde::{Serialize, Deserialize};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockchainMessage {
    pub r#type: String,
    pub data: serde_json::Value,
    pub signature: Option<String>,
}

impl BlockchainMessage {
    pub fn new(r#type: &str, data: serde_json::Value) -> Self {
        BlockchainMessage {
            r#type: r#type.to_string(),
            data,
            signature: None,
        }
    }

    pub fn sign(&mut self, signing_key: &SigningKey) {
        let message_data = serde_json::to_string(&self.data).unwrap();
        let signature: Signature = signing_key.sign(message_data.as_bytes());
        self.signature = Some(STANDARD.encode(signature.to_bytes().as_slice()));

    }

    pub fn verify_signature(&self, verifying_key: &VerifyingKey) -> bool {
        if let Some(signature) = &self.signature {
            if let Ok(signature_bytes) = STANDARD.decode(signature) {
                if let Ok(sig) = Signature::from_der(&signature_bytes) {
                    let message_data = serde_json::to_string(&self.data).unwrap();
                    return verifying_key.verify(message_data.as_bytes(), &sig).is_ok();
                }
            }
        }
        false
    }
}
