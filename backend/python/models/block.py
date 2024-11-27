from dataclasses import dataclass
from typing import List, Optional
from datetime import datetime

@dataclass
class Transaction:
    sender: str
    recipient: str
    amount: float

@dataclass
class Block:
    index: int
    timestamp: datetime
    transactions: List[Transaction]
    previous_hash: str
    nonce: int = 0
    hash: Optional[str] = None
