from models.block import Block, Transaction
from utils.crypto import hash_block, generate_keys
from datetime import datetime, timezone

class Blockchain:
    def __init__(self):
        self.chain = []
        self.pending_transactions = []
        self.create_genesis_block()

    def create_genesis_block(self):
        """Create the first block in the chain."""
        genesis_block = Block(
            index=0,
            timestamp=datetime.now(timezone.utc),
            transactions=[],
            previous_hash="0",
            nonce=0
        )
        genesis_block.hash = hash_block(str(genesis_block))
        self.chain.append(genesis_block)

    def add_transaction(self, sender, recipient, amount):
        """Add a transaction to the pending transactions list."""
        transaction = Transaction(sender, recipient, amount)
        self.pending_transactions.append(transaction)

    def mine_block(self):
        """Create a new block with pending transactions."""
        previous_block = self.chain[-1]
        new_block = Block(
            index=len(self.chain),
            timestamp=datetime.now(timezone.utc),
            transactions=self.pending_transactions,
            previous_hash=previous_block.hash,
            nonce=0
        )
        new_block.hash = hash_block(str(new_block))
        self.chain.append(new_block)
        self.pending_transactions = []

# Example usage
if __name__ == "__main__":
    blockchain = Blockchain()
    blockchain.add_transaction("Alice", "Bob", 10.0)
    blockchain.mine_block()
    print(f"Blockchain: {blockchain.chain}")
