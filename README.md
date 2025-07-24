# SolDocsPlus — Decentralized Wallet-to-Wallet Encrypted Document Exchange on Solana

Secure wallet-to-wallet encrypted document sharing on Solana
Project Overview
SolDocsPlus is a privacy-first decentralized application (DApp) built on the Solana blockchain that enables secure, wallet-to-wallet encrypted document sharing without relying on centralized intermediaries.

Using modern blockchain techniques and cryptography, SolDocsPlus allows users to:

Upload and encrypt important documents (legal contracts, medical records, certificates, images, PDFs) on their device.

Store encrypted files securely on decentralized storage like IPFS or Arweave.

Share documents directly to another Solana wallet address with on-chain metadata logging for traceability and access control.

Manage access rights with the ability to grant, revoke, or set expiration on document access — all enforced by smart contracts.

Benefit from a transparent, tamper-proof, and auditable document transfer history secured by Solana’s fast and low-cost network.

Why SolDocsPlus?
Traditional document sharing methods like email or cloud storage services come with:

Privacy risks (data breaches, leaks)

Centralized points of failure

Limited control over who accesses your sensitive files

SolDocsPlus leverages blockchain’s decentralization and cryptographic guarantees to solve these problems by giving full control and ownership to users. Every document transfer is:

Encrypted end-to-end: Files are encrypted locally before upload, so no one else can read them.

Auditable on-chain: Metadata like document hashes, access grants, and timestamps are stored on Solana, providing an immutable history.

Access-controlled: Senders can revoke or limit access anytime.

Gas-efficient: Solana’s fast and low-fee network makes frequent access logging and revocation practical

| Layer                   | Technology                          | Purpose/Role                                              |
| ----------------------- | ----------------------------------- | --------------------------------------------------------- |
| 🔗 **Blockchain Layer** | **Solana**                          | High-performance blockchain for on-chain logic            |
| 📜 **Smart Contracts**  | **Rust + Anchor**                   | Build, deploy, and manage Solana smart contracts          |
| 🧑‍💻 **Frontend**      | **React + Vite or Next.js**         | Build fast UI for upload, access control, decryption      |
| 🧠 **CLI Toolkit**      | **sol-dev CLI**                     | Scaffolding + SDK generation for Solana programs          |
| 💾 **Storage**          | **IPFS / Arweave**                  | Off-chain, decentralized storage of encrypted files       |
| 🔐 **Encryption**       | **AES-GCM (Web Crypto API)**        | Encrypt files client-side in the browser                  |
| 👛 **Wallet Adapter**   | **Solana Wallet Adapter + Phantom** | Connect wallets to the DApp, sign transactions            |
| 🔄 **Automation**       | **Node.js (cron script)**           | Run cleanup/revocation for expired document access        |
| 📦 **SDK**              | **Generated TypeScript SDK**        | Use Anchor program from frontend using `sol-dev generate` |

Detailed Breakdown
🔗 Blockchain Layer
Solana: Chosen for its low fees and high speed.

Devnet/Mainnet compatibility for testing and launch.

📜 Smart Contracts
Rust: The main language for Solana programs.

Anchor: Framework that simplifies account validation, error handling, and instruction parsing.

Handles:

Document registration

Granting/revoking access

Logging file opens (audit)

🧠 sol-dev CLI
Bootstrap the project

Generate SDK from Anchor program

Connect smart contract to frontend

💻 Frontend
React: Single-page app for UX

Vite or Next.js: Fast dev environment

Built UI for:

Uploading and encrypting files

Viewing shared documents

Managing access

🔐 Encryption
Uses Web Crypto API (built-in browser tools)

Encrypts files using AES-GCM

Can optionally derive key from wallet signature (window.solana.signMessage())

💾 Off-chain Storage
IPFS or Arweave:

IPFS via Infura, Web3.Storage, or Pinata

Arweave for permanent, pay-once storage

Encrypted content is uploaded, and CID stored on-chain

👛 Wallet
Phantom Wallet

@solana/wallet-adapter-react

Used for:

Signing transactions

Signing messages (optional for key derivation)

Identifying sender/recipient

🔄 Automation
scripts/cleanup.ts:

Uses the SDK

Scans documents with expired timestamps

Calls smart contract to revoke access

| Tool            | Use                                       |
| --------------- | ----------------------------------------- |
| `solana-cli`    | Wallets, keys, deploy programs            |
| `anchor-cli`    | Build, test, and deploy smart contracts   |
| `yarn` or `npm` | Frontend package manager                  |
| GitHub          | Version control and open-source hosting   |
| GitHub Actions  | Automate test and deploy flows (optional) |


| Feature                           | Description                                                                                    |
| --------------------------------- | ---------------------------------------------------------------------------------------------- |
| Wallet-to-wallet document sharing | Send encrypted documents directly between Solana wallets without intermediaries.               |
| End-to-end encryption             | Client-side AES-GCM encryption ensures only intended recipients can decrypt.                   |
| Decentralized storage             | Files stored off-chain on IPFS or Arweave for persistence and scalability.                     |
| On-chain metadata & logging       | All document metadata, access grants, revocations, and access logs stored immutably on Solana. |
| Access control & revocation       | Senders can grant, revoke, or set expiration on document access rights anytime.                |
| Transparent audit trail           | Every document transfer is logged on-chain, ensuring accountability and compliance.            |

| Technology         | Role                                                                 |
| ------------------ | -------------------------------------------------------------------- |
| **Solana**         | High-performance blockchain for on-chain metadata and access control |
| **Anchor (Rust)**  | Smart contract framework for program development                     |
| **React**          | Frontend UI for file upload, encryption, and access management       |
| **sol-dev CLI**    | Project scaffolding and deployment                                   |
| **IPFS / Arweave** | Decentralized storage for encrypted files                            |
| **Phantom Wallet** | User authentication and wallet interactions                          |
| **AES-GCM**        | Encryption algorithm for client-side data security                   |

How It Works
Upload & Encrypt: User selects a document and encrypts it locally in the browser.

Store Off-chain: The encrypted file is uploaded to IPFS/Arweave, returning a content identifier (CID).

Register Document: Metadata including the CID, document hash, owner, and access rules are stored on Solana via Anchor smart contract.

Grant Access: Owner grants access to other wallet addresses on-chain.

View & Decrypt: Authorized recipients retrieve the encrypted file, then decrypt locally using shared keys.

Access Logging: Every access event is logged on-chain to maintain an immutable audit trail.

Revoke Access: Owners can revoke or expire access anytime.

Potential Use Cases
Legal firms: Share sensitive contracts with clients securely and verifiably.

Medical sector: Exchange medical records with strict privacy and control.

Education: Verify and share diplomas or certificates.

Enterprises: Exchange confidential documents with partners without email risks.

General users: Private file sharing with friends or family using just wallet addresses.

 Optional Enhancements
NFT integration: Convert access grants into soulbound NFTs (non-transferable).

ZK Proofs: Prove that a document was shared without revealing its contents.

Multi-file encryption: Batch uploads.

Solana Name Service (SNS): Send docs to usernames instead of wallet addresses.
