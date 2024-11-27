from fastapi import APIRouter
from services.blockchain import get_blockchain_data

router = APIRouter()

@router.get("/")
async def get_blockchain_info():
    data = get_blockchain_data()
    return {"info": data}
