```
pyrust-blockchain/
├── backend/             
│   ├── python/             # Python Backend (main backend logic)
│   │   ├── app.py          # FastAPI entry point
│   │   ├── api/            # REST/GraphQL API routes
|   │   |   ├── __init__.py   # Makes the directory a package
│   |   |   └── blockchain.py # Example routes related to blockchain
│   │   ├── models/         # Data models (e.g., transactions, blocks)
|   |   |   ├── __init__.py   
|   |   |   └── blockchain.py
│   │   ├── utils/          # Helper functions
|   |   |   ├── __init__.py   
|   |   |   └── common.py
│   │   ├── services/       # Python services for blockchain logic
|   |   |   ├── __init__.py   
|   |   |   └── blockchain.py
│   │   └── requirements.txt# Python dependencies
│   ├── rust/               # Rust Backend (performance-critical tasks)
│   │   ├── src/
│   │   │   ├── lib.rs      # Rust library entry point
│   │   │   ├── crypto/     # Cryptography utilities (hashing, signing)
│   │   │   ├── consensus/  # Consensus mechanism implementation
│   │   │   ├── ffi/        # Rust <-> Python FFI bindings
│   │   └── Cargo.toml      # Rust dependencies
│   ├── Dockerfile          # Unified backend Docker configuration
├── frontend/            
│   ├── app/                # Next.js app router
|   |   ├── page.tsx        # Next.js page component
│   │   ├── layout.tsx      # Next.js layout component
│   ├── components/         # UI components
│   ├── public/             # Static assets
|   ├── eslintrc.json          
│   ├── tailwind.config.ts  # Tailwind CSS configuration
│   └── package.json        # Node.js dependencies
├── scripts/             
│   ├── seed_network.py     # Network bootstrapping (Python)
│   └── analyze_chain.py    # Chain analysis (Python)
├── docker-compose.yml      # Orchestration for all services
└── README.md               # Documentation
```

```bash
make develop         # Build Rust workspace and install Rust modules in Python
make build           # Compile Rust workspace
make test            # Run all Rust tests
make clean           # Clean Rust and Python artifacts
make run-python      # Run a Python script
make run-fastapi     # Run the FastAPI server
```