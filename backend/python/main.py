# from fastapi import FastAPI
# from api.blockchain import router as blockchain_router
# from services.blockchain import get_blockchain_data
# from utils.crypto import hash_block, generate_keys

# app = FastAPI()

# # Include routes
# app.include_router(blockchain_router)
# app.include_router(blockchain_router, prefix="/blockchain")

# @app.get("/status")
# async def read_status():
#     return {"status": "Python backend is running"}

# @app.get("/services")
# async def read_services():
#     data = get_blockchain_data()
#     return {"services": data}

# @app.get("/keys")
# async def read_keys():
#     private_key, public_key = generate_keys()
#     return {"private_key": private_key, "public_key": public_key}

from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
import httpx

app = FastAPI()

RUST_SERVER_URL = "http://localhost:8080"

origins = [
    "http://localhost:3000",  # Next.js development server
    "https://your-production-domain.com"  # Production domain
]

app.add_middleware(
    CORSMiddleware,
    allow_origins=origins,
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

@app.get("/")
async def get_routes():
    try:
        async with httpx.AsyncClient() as client:
            response = await client.get(f"{RUST_SERVER_URL}/")
            return response.json()
    except httpx.RequestError as e:
        return {"error": str(e)}
    except Exception as e:
        return {"error": str(e)}

@app.get("/generate_key_pair")
async def generate_key_pair():
    async with httpx.AsyncClient() as client:
        response = await client.get(f"{RUST_SERVER_URL}/generate_key_pair")
        return response.json()
