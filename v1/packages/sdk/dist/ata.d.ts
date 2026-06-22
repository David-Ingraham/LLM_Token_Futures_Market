import { PublicKey } from "@solana/web3.js";
export declare const ASSOCIATED_TOKEN_PROGRAM_ID: PublicKey;
export declare const TOKEN_PROGRAM_ID: PublicKey;
/** Same address as @solana/spl-token getAssociatedTokenAddressSync (browser-safe). */
export declare function getAssociatedTokenAddressSync(mint: PublicKey, owner: PublicKey, allowOwnerOffCurve?: boolean): PublicKey;
