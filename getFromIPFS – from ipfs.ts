export async function getFromIPFS(cid: string): Promise<Blob> {
  const res = await fetch(`https://${cid}.ipfs.w3s.link/encrypted-file`);
  const blob = await res.blob();
  return blob;
}
