from fastapi import FastAPI
from api.blockchain import router as blockchain_router

app = FastAPI()

# Include routes
app.include_router(blockchain_router, prefix="/blockchain")

@app.get("/status")
async def read_status():
    return {"status": "Python backend is running"}

