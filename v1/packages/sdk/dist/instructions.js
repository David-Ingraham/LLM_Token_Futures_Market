import { PublicKey, TransactionInstruction } from "@solana/web3.js";
import { encodeInstruction, ixData, writeI64, writeU16, writeU64, writeU8, } from "./bytes.js";
import { PROGRAM_ID } from "./constants.js";
const TOKEN_PROGRAM = new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
const SYSTEM_PROGRAM = new PublicKey("11111111111111111111111111111111");
const RENT_SYSVAR = new PublicKey("SysvarRent111111111111111111111111111111111");
const INIT_MARKET_DATA_LEN = 1 + 8 * 3 + 8 * 3 + 2;
export function initializeMarket(accounts, params) {
    const data = encodeInstruction(INIT_MARKET_DATA_LEN, (buf) => {
        let o = 0;
        o = writeU8(buf, o, 0);
        o = writeU64(buf, o, params.marketId);
        o = writeU64(buf, o, params.entryPriceMicro);
        o = writeU64(buf, o, params.maxSettlementPriceMicro);
        o = writeI64(buf, o, params.openTs);
        o = writeI64(buf, o, params.tradeCutoffTs);
        o = writeI64(buf, o, params.expiryTs);
        return writeU16(buf, o, params.feeBps);
    });
    return new TransactionInstruction({
        programId: PROGRAM_ID,
        keys: [
            { pubkey: accounts.authority, isSigner: true, isWritable: true },
            { pubkey: accounts.oracle, isSigner: false, isWritable: false },
            { pubkey: accounts.mint, isSigner: false, isWritable: false },
            { pubkey: accounts.market, isSigner: false, isWritable: true },
            { pubkey: accounts.vault, isSigner: false, isWritable: true },
            { pubkey: TOKEN_PROGRAM, isSigner: false, isWritable: false },
            { pubkey: SYSTEM_PROGRAM, isSigner: false, isWritable: false },
            { pubkey: RENT_SYSVAR, isSigner: false, isWritable: false },
        ],
        data: ixData(data),
    });
}
export function openPosition(accounts, side, contracts) {
    const data = encodeInstruction(1 + 1 + 8, (buf) => {
        let o = 0;
        o = writeU8(buf, o, 1);
        o = writeU8(buf, o, side);
        return writeU64(buf, o, contracts);
    });
    return new TransactionInstruction({
        programId: PROGRAM_ID,
        keys: [
            { pubkey: accounts.owner, isSigner: true, isWritable: true },
            { pubkey: accounts.market, isSigner: false, isWritable: false },
            { pubkey: accounts.position, isSigner: false, isWritable: true },
            { pubkey: accounts.ownerTokenAccount, isSigner: false, isWritable: true },
            { pubkey: accounts.vault, isSigner: false, isWritable: true },
            { pubkey: TOKEN_PROGRAM, isSigner: false, isWritable: false },
            { pubkey: SYSTEM_PROGRAM, isSigner: false, isWritable: false },
            { pubkey: RENT_SYSVAR, isSigner: false, isWritable: false },
        ],
        data: ixData(data),
    });
}
export function haltTrading(market, crank) {
    return new TransactionInstruction({
        programId: PROGRAM_ID,
        keys: [
            { pubkey: crank, isSigner: true, isWritable: false },
            { pubkey: market, isSigner: false, isWritable: true },
        ],
        data: ixData(Uint8Array.from([2])),
    });
}
export function postSettlementPrice(oracle, market, settlementPriceMicro) {
    const data = encodeInstruction(1 + 8, (buf) => {
        let o = writeU8(buf, 0, 3);
        return writeU64(buf, o, settlementPriceMicro);
    });
    return new TransactionInstruction({
        programId: PROGRAM_ID,
        keys: [
            { pubkey: oracle, isSigner: true, isWritable: false },
            { pubkey: market, isSigner: false, isWritable: true },
        ],
        data: ixData(data),
    });
}
export function settlePosition(accounts) {
    return new TransactionInstruction({
        programId: PROGRAM_ID,
        keys: [
            { pubkey: accounts.owner, isSigner: true, isWritable: true },
            { pubkey: accounts.market, isSigner: false, isWritable: false },
            { pubkey: accounts.position, isSigner: false, isWritable: true },
            { pubkey: accounts.ownerTokenAccount, isSigner: false, isWritable: true },
            { pubkey: accounts.vault, isSigner: false, isWritable: true },
            { pubkey: TOKEN_PROGRAM, isSigner: false, isWritable: false },
        ],
        data: ixData(Uint8Array.from([4])),
    });
}
