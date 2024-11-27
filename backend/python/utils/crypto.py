from rust_crypto import generate_key_pair, hash_data

def generate_keys():
    """Generate public and private keys using Rust."""
    private_key, public_key = generate_key_pair()
    return private_key, public_key

def hash_block(block_data: str) -> str:
    """Hash block data using Rust."""
    return hash_data(block_data)
