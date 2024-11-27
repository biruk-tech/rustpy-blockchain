// use ed25519_dalek::{Keypair, Signer};
use pyo3::prelude::*;
// use base64;
// use sha2::{Digest, Sha256};
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use rand::rngs::OsRng; // Use OsRng for cryptographic randomness
use hex; // Import hex crate

// #[pyfunction]
// fn generate_key_pair() -> PyResult<(String, String)> {
//     let mut csprng = rand::thread_rng(); // Use thread_rng instead of OsRng
//     let keypair = Keypair::generate(&mut csprng);
//     let private_key = base64::encode(keypair.secret.to_bytes());
//     let public_key = base64::encode(keypair.public.to_bytes());
//     Ok((public_key, private_key))
// }

#[pyfunction]
fn generate_secp256k1_keypair() -> (SecretKey, PublicKey) {
    let secp = Secp256k1::new();
    let mut rng = OsRng; // Define rng
    let (secret_key, public_key) = secp.generate_keypair(&mut rng); // Ensure this line is correct
    // Convert keys to strings or bytes
    let secret_key_hex = hex::encode(secret_key.to_bytes()); // Convert SecretKey to hex string
    let public_key_hex = hex::encode(public_key.to_bytes()); // Convert PublicKey to hex string
    Ok((secret_key_hex, public_key_hex)) // Return as strings
}

// #[pyfunction]
// fn hash_data(data: &str) -> PyResult<String> {
//     let mut hasher = Sha256::new();
//     hasher.update(data.as_bytes());
//     Ok(format!("{:x}", hasher.finalize()))
// }

// #[pyfunction]
// fn sign_message(message: &str, private_key: &str) -> PyResult<Vec<u8>> {
//     let secret_key_bytes = base64::decode(private_key).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid private key: {}", e)))?;
//     let secret_key = ed25519_dalek::SecretKey::from_bytes(&secret_key_bytes).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid private key format: {}", e)))?;
//     let public_key = ed25519_dalek::PublicKey::from(&secret_key);
//     let keypair = Keypair { secret: secret_key, public: public_key };
//     let signature = keypair.sign(message.as_bytes());
//     Ok(signature.to_bytes().to_vec())
// }

#[pymodule]
fn crypto(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(generate_secp256k1_keypair, py)?)?;
    // m.add_function(wrap_pyfunction!(generate_key_pair, m)?)?;
    // m.add_function(wrap_pyfunction!(hash_data, m)?)?;
    // m.add_function(wrap_pyfunction!(sign_message, m)?)?;
    Ok(())
}

