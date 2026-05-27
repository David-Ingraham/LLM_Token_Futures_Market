export type DevnetConfig = {
  cluster: string;
  rpc: string;
  programId: string;
  mint: string;
  market: string;
  vault: string;
  authority: string;
  oracle: string;
  marketId: string;
  entryPriceMicro: string;
  tradeCutoffTs: string;
  expiryTs: string;
};

export async function loadDevnetConfig(): Promise<DevnetConfig | null> {
  try {
    const res = await fetch("/devnet.json");
    if (!res.ok) return null;
    return (await res.json()) as DevnetConfig;
  } catch {
    return null;
  }
}
