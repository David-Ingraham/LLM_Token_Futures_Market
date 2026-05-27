import { PublicKey } from "@solana/web3.js";

export const ASSOCIATED_TOKEN_PROGRAM_ID = new PublicKey(
  "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL",
);

export const TOKEN_PROGRAM_ID = new PublicKey(
  "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
);

/** Same address as @solana/spl-token getAssociatedTokenAddressSync (browser-safe). */
export function getAssociatedTokenAddressSync(
  mint: PublicKey,
  owner: PublicKey,
  allowOwnerOffCurve = false,
): PublicKey {
  if (!allowOwnerOffCurve && !PublicKey.isOnCurve(owner.toBytes())) {
    throw new Error("Owner cannot sign");
  }
  const [address] = PublicKey.findProgramAddressSync(
    [owner.toBytes(), TOKEN_PROGRAM_ID.toBytes(), mint.toBytes()],
    ASSOCIATED_TOKEN_PROGRAM_ID,
  );
  return address;
}
