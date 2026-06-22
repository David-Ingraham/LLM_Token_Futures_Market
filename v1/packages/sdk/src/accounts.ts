import { Connection, PublicKey } from "@solana/web3.js";
import { readI64, readU16, readU64, readU8 } from "./bytes.js";
import { MARKET_MAGIC, POSITION_MAGIC } from "./constants.js";

export type MarketAccount = {
  pubkey: PublicKey;
  magic: bigint;
  authority: PublicKey;
  oracle: PublicKey;
  mint: PublicKey;
  vault: PublicKey;
  marketId: bigint;
  entryPriceMicro: bigint;
  maxSettlementPriceMicro: bigint;
  settlementPriceMicro: bigint;
  openTs: bigint;
  tradeCutoffTs: bigint;
  expiryTs: bigint;
  feeBps: number;
  contractMtok: bigint;
  status: number;
  bump: number;
  vaultBump: number;
};

export type PositionAccount = {
  pubkey: PublicKey;
  magic: bigint;
  owner: PublicKey;
  market: PublicKey;
  side: number;
  contracts: bigint;
  lockedMargin: bigint;
  settled: boolean;
  bump: number;
};

export function decodeMarket(data: Uint8Array, pubkey: PublicKey): MarketAccount | null {
  if (data.length < 197) return null;
  let o = 0;
  const magic = readU64(data, o);
  o += 8;
  if (magic !== MARKET_MAGIC) return null;
  const authority = new PublicKey(data.slice(o, o + 32));
  o += 32;
  const oracle = new PublicKey(data.slice(o, o + 32));
  o += 32;
  const mint = new PublicKey(data.slice(o, o + 32));
  o += 32;
  const vault = new PublicKey(data.slice(o, o + 32));
  o += 32;
  const marketId = readU64(data, o);
  o += 8;
  const entryPriceMicro = readU64(data, o);
  o += 8;
  const maxSettlementPriceMicro = readU64(data, o);
  o += 8;
  const settlementPriceMicro = readU64(data, o);
  o += 8;
  const openTs = readI64(data, o);
  o += 8;
  const tradeCutoffTs = readI64(data, o);
  o += 8;
  const expiryTs = readI64(data, o);
  o += 8;
  const feeBps = readU16(data, o);
  o += 2;
  const contractMtok = readU64(data, o);
  o += 8;
  const status = readU8(data, o);
  o += 1;
  const bump = readU8(data, o);
  o += 1;
  const vaultBump = readU8(data, o);

  return {
    pubkey,
    magic,
    authority,
    oracle,
    mint,
    vault,
    marketId,
    entryPriceMicro,
    maxSettlementPriceMicro,
    settlementPriceMicro,
    openTs,
    tradeCutoffTs,
    expiryTs,
    feeBps,
    contractMtok,
    status,
    bump,
    vaultBump,
  };
}

export function decodePosition(data: Uint8Array, pubkey: PublicKey): PositionAccount | null {
  if (data.length < 91) return null;
  let o = 0;
  const magic = readU64(data, o);
  o += 8;
  if (magic !== POSITION_MAGIC) return null;
  const owner = new PublicKey(data.slice(o, o + 32));
  o += 32;
  const market = new PublicKey(data.slice(o, o + 32));
  o += 32;
  const side = readU8(data, o);
  o += 1;
  const contracts = readU64(data, o);
  o += 8;
  const lockedMargin = readU64(data, o);
  o += 8;
  const settled = readU8(data, o) !== 0;
  o += 1;
  const bump = readU8(data, o);

  return {
    pubkey,
    magic,
    owner,
    market,
    side,
    contracts,
    lockedMargin,
    settled,
    bump,
  };
}

export async function fetchMarket(
  connection: Connection,
  address: PublicKey,
): Promise<MarketAccount | null> {
  const info = await connection.getAccountInfo(address);
  if (!info?.data) return null;
  return decodeMarket(new Uint8Array(info.data), address);
}

export async function fetchPosition(
  connection: Connection,
  address: PublicKey,
): Promise<PositionAccount | null> {
  const info = await connection.getAccountInfo(address);
  if (!info?.data) return null;
  return decodePosition(new Uint8Array(info.data), address);
}
