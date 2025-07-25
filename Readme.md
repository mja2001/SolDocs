# 🔐 SolCipher – Secure Encrypted Documents Sharing on Solana
SolCipher is a decentralized application that allows users to upload, encrypt, and share documents from wallet to wallet — all secured with client-side encryption, on-chain access control, and decentralized storage using IPFS.

---

## 🚀 Features

- ✅ End-to-end file encryption (AES-GCM in browser)
- ✅ Upload files to IPFS via Web3.Storage
- ✅ Share access with specific Solana wallets
- ✅ Enforce access rules on-chain using Anchor smart contracts
- ✅ Revoke or auto-expire document access
- ✅ CLI script to clean up expired document access

---

## 🛠️ Tech Stack

| Layer           | Tech                      | Description                                  |
|----------------|---------------------------|----------------------------------------------|
| 🔗 Blockchain   | Solana                    | Fast, low-cost layer for contract logic      |
| 📜 Smart Contract | Rust + Anchor            | Manages documents, access, and revocation    |
| 💻 Frontend     | React + Vite              | Simple DApp interface for upload & decrypt   |
| 🔐 Encryption   | Web Crypto API (AES-GCM)  | Encrypts files in the browser                |
| 💾 Storage      | IPFS (via Web3.Storage)   | Stores encrypted files                       |
| 👛 Wallet       | Phantom + Wallet Adapter  | For auth, signing, and key generation        |
| 🔁 Automation   | Node.js Script            | Scans for and revokes expired file access    |

---

## 📦 Architecture

```
User Uploads File
|
\|---> Encrypted in Browser (AES-GCM)
\|---> Uploaded to IPFS (Web3.Storage)
\|---> Metadata stored on-chain (Solana)
|
\--> Access granted to target wallet
\--> Expiry timestamp managed
```

---

## 🌐 Pages

- **Upload Page**: Select → Encrypt → IPFS Upload → Smart contract registration  
- **View Page**: Check access → Download encrypted file → Decrypt with wallet key  
- **Cleanup Script**: Revokes expired access via CLI or scheduled cron

## 🧑‍💻 Getting Started

### 1️⃣ Prerequisites

- Node.js `v18+`
- Yarn or npm
- Solana CLI + Anchor CLI
- Phantom Wallet installed
- Web3.Storage account

### 2️⃣ Anchor Program Setup

```bash
cd programs/solcipher
anchor build
anchor deploy
```

Make sure to replace the deployed `programId` in the frontend.

### 3️⃣ React Frontend Setup

```bash
cd app
yarn install
yarn dev
```

Visit: [http://localhost:3000](http://localhost:3000)

### 4️⃣ IPFS Setup

- [Create a Web3.Storage account](https://web3.storage)
- Get your API key
- Replace it in `app/src/utils/ipfs.ts`

### 5️⃣ Run Cleanup Script

```bash
cd scripts
yarn cleanup
```

You can schedule it to run daily with `cron` for automatic access revocation.

---

## 📂 Project Structure

```
solcipher/
├── app/                  # React frontend
│   ├── pages/Upload.tsx
│   ├── pages/View.tsx
│   └── utils/
├── programs/solcipher/   # Anchor smart contract
│   └── src/lib.rs
├── scripts/cleanup.ts    # Revoke expired access
├── idl/                  # Anchor IDL
├── README.md             # You're here!
```

---

## 🔐 Security Highlights

- Files are encrypted **before** leaving the user's device
- Keys derived from **wallet signatures**
- Decryption only possible with private key (no backend)
- Access control enforced **on-chain**

---

## 💡 Use Cases

- Legal contract exchange
- Medical records or health data
- Government ID or KYC files
- Blockchain-based private messaging
- NFT-gated document access (future extension)

---

## 💬 Contributing

Want to contribute?

```bash
git clone https://github.com/YOUR_USERNAME/solcipher.git
```

- Fork & PRs welcome
- Bugs & feedback? Open an issue

---

## 🧠 Project Idea Summary

SolCipher solves the problem of **secure file sharing in a decentralized world**. It’s like Google Drive, but:

- You own the files
- No centralized servers
- Only the recipient can view the file
- Access is verifiable on-chain

---

## 📜 License

MIT License — use freely with attribution.

---

## 🌍 Connect with Me

- GitHub: [mja_2001](https://github.com/mja2001)
- LinkedIn: [alayham m almajali](http://www.linkedin.com/in/alayham-m-almajali-343286278)
- Twitter: [@aymmjsol](https://twitter.com/aymmj2001)
...
