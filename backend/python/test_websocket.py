import asyncio
import websockets

async def test_websocket():
    uri = "ws://127.0.0.1:8081"
    async with websockets.connect(uri) as websocket:
        print("Connected to server programmatically using Python!")
        await websocket.send("Hello, server from Python backend!")
        response = await websocket.recv()
        print(f"Message from server: {response}")

asyncio.run(test_websocket())
