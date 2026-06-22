import { PublicKey } from "@solana/web3.js";
/** Deployed program id (devnet keypair in target/deploy). */
export const PROGRAM_ID = new PublicKey("9EBRakVzKCaHBUXdC8DMcUx9Seyf7o48eJS1wbN6ogpD");
export const MICRO_PER_UNIT = 1_000_000;
export const MARKET_MAGIC = 0x4d4b545f4c544631n;
export const POSITION_MAGIC = 0x504f535f4c544631n;
export const Side = { Long: 0, Short: 1 };
export const MarketStatus = { Open: 0, Halted: 1, PricePosted: 2 };
