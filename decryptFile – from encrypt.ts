export async function decryptFile(
  encryptedBlob: Blob,
  keyBytes: Uint8Array,
  iv: Uint8Array
): Promise<Blob> {
  const encryptedBuffer = await encryptedBlob.arrayBuffer();
  const key = await crypto.subtle.importKey('raw', keyBytes, 'AES-GCM', false, ['decrypt']);

  const decrypted = await crypto.subtle.decrypt({ name: 'AES-GCM', iv }, key, encryptedBuffer);
  return new Blob([decrypted]);
}
