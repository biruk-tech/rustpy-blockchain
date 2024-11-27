use k256::{ecdsa::{SigningKey, VerifyingKey, signature::Signer}, EncodedPoint};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;

/// Command-line interface
#[derive(Parser)]
#[command(name = "crypto_tool")]
#[command(about = "Cryptographic operations using secp256k1", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Available commands
#[derive(Subcommand)]
enum Commands {
    GenerateKeys { output: String },
    SignMessage { message: String, private_key: String, output: String },
    VerifySignature { message: String, signature: String, public_key: String },
}

/// Key pair structure
#[derive(Serialize, Deserialize)]
struct KeyPair {
    public_key: String,
    private_key: String,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::GenerateKeys { output } => {
            let signing_key = SigningKey::random(&mut rand::thread_rng());
            let verifying_key = signing_key.verifying_key();
            let keypair = KeyPair {
                public_key: base64::encode(verifying_key.to_encoded_point(false).as_bytes()),
                private_key: base64::encode(signing_key.to_bytes()),
            };
            fs::write(output, serde_json::to_string(&keypair).unwrap()).expect("Failed to write keypair to file");
        }
        Commands::SignMessage { message, private_key, output } => {
            let private_key_bytes = base64::decode(&private_key).expect("Invalid Base64 private key");
            let signing_key = SigningKey::from_bytes(&private_key_bytes).expect("Invalid private key");
            let signature = signing_key.sign(message.as_bytes());
            fs::write(output, base64::encode(signature.as_bytes())).expect("Failed to write signature to file");
        }
        Commands::VerifySignature { message, signature, public_key } => {
            let signature_bytes = base64::decode(signature).expect("Invalid Base64 signature");
            let public_key_bytes = base64::decode(public_key).expect("Invalid Base64 public key");
            let verifying_key = VerifyingKey::from_encoded_point(&EncodedPoint::from_bytes(&public_key_bytes).unwrap()).unwrap();
            let signature = k256::ecdsa::Signature::from_bytes(&signature_bytes).unwrap();
            let is_valid = verifying_key.verify(message.as_bytes(), &signature).is_ok();
            println!("{}", is_valid);
        }
    }
}
