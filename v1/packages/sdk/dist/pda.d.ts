import { PublicKey } from "@solana/web3.js";
export declare function marketPda(authority: PublicKey, marketId: bigint): [PublicKey, number];
export declare function vaultPda(market: PublicKey): [PublicKey, number];
export declare function positionPda(market: PublicKey, owner: PublicKey): [PublicKey, number];
