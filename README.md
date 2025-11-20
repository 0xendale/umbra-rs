Umbra Project — README (Draft)

Umbra is a personal project where I’m exploring the idea of anonymous / stealth payments on Solana. The core concept: a sender can transfer funds without revealing the real receiving wallet, while the recipient can automatically sweep those funds into their main wallet.

This is a short draft README to describe the high-level idea — not deep technical details. I will continue updating it as the project evolves.

⸻

🚀 Project Goals
• Enable private fund receiving with stealth-like addresses.
• Make payments unlinkable on-chain.
• Automatically scan the blockchain for incoming stealth outputs.
• Automatically sweep SOL/SPL to the user’s real wallet.
• Provide a clean and simple SDK for integration.

⸻

🔧 Main Development Phases

(1) Basic Cryptography
Generate identities, compute temporary keys, and validate stealth outputs.

(2) RPC Scanning
Read blockchain data and detect outputs that belong to the user.

(3) Sweeping
Use the spend key to transfer funds into the real wallet.

(4) Storage
Store identities, keys, and scan results.

(5) SDK / Integration
Expose a simple API for apps and services.

⸻

📌 Project Status

The project is actively in development and many parts are still experimental.
This README will be updated as progress continues.

⸻

👤 Author

Endale (0xendale) — 2025
