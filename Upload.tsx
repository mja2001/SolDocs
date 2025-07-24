import React, { useState } from 'react';
import { useWallet } from '@solana/wallet-adapter-react';
import { registerDocument } from '../utils/solanaClient'; // Your Solana client SDK
import { encryptFile } from '../utils/encrypt';
import { uploadToIPFS } from '../utils/ipfs';

const Upload = () => {
  const { publicKey, signMessage } = useWallet();
  const [file, setFile] = useState<File | null>(null);
  const [status, setStatus] = useState('');

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files) {
      setFile(e.target.files[0]);
    }
  };

  const handleUpload = async () => {
    if (!file || !publicKey || !signMessage) {
      alert('Wallet or file missing');
      return;
    }

    setStatus('Encrypting file...');
    const encryptionKey = await signMessage(new TextEncoder().encode('doc-encryption-key'));

    const { encryptedBlob, iv } = await encryptFile(file, encryptionKey);

    setStatus('Uploading to IPFS...');
    const ipfsHash = await uploadToIPFS(encryptedBlob);

    setStatus('Registering on-chain...');
    const expiresInDays = 7;
    const expiresAt = Math.floor(Date.now() / 1000) + expiresInDays * 86400;

    await registerDocument(ipfsHash, expiresAt);

    setStatus('✅ Document uploaded and registered!');
  };

  return (
    <div style={{ padding: '2rem' }}>
      <h2>📄 Upload & Share Secure Document</h2>
      <input type="file" onChange={handleFileChange} />
      <button onClick={handleUpload} disabled={!file}>
        Upload & Encrypt
      </button>
      <p>{status}</p>
    </div>
  );
};

export default Upload;
