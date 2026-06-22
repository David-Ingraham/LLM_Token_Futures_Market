import { PublicKey } from "@solana/web3.js";
import { utf8Bytes, u64le } from "./bytes.js";
import { PROGRAM_ID } from "./constants.js";
export function marketPda(authority, marketId) {
    return PublicKey.findProgramAddressSync([utf8Bytes("market"), authority.toBytes(), u64le(marketId)], PROGRAM_ID);
}
export function vaultPda(market) {
    return PublicKey.findProgramAddressSync([utf8Bytes("vault"), market.toBytes()], PROGRAM_ID);
}
export function positionPda(market, owner) {
    return PublicKey.findProgramAddressSync([utf8Bytes("position"), market.toBytes(), owner.toBytes()], PROGRAM_ID);
}
