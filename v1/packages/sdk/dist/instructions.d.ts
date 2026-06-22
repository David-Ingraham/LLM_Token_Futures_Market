import { PublicKey, TransactionInstruction } from "@solana/web3.js";
export type InitMarketParams = {
    marketId: bigint;
    entryPriceMicro: bigint;
    maxSettlementPriceMicro: bigint;
    openTs: bigint;
    tradeCutoffTs: bigint;
    expiryTs: bigint;
    feeBps: number;
};
export declare function initializeMarket(accounts: {
    authority: PublicKey;
    oracle: PublicKey;
    mint: PublicKey;
    market: PublicKey;
    vault: PublicKey;
}, params: InitMarketParams): TransactionInstruction;
export declare function openPosition(accounts: {
    owner: PublicKey;
    market: PublicKey;
    position: PublicKey;
    ownerTokenAccount: PublicKey;
    vault: PublicKey;
}, side: number, contracts: bigint): TransactionInstruction;
export declare function haltTrading(market: PublicKey, crank: PublicKey): TransactionInstruction;
export declare function postSettlementPrice(oracle: PublicKey, market: PublicKey, settlementPriceMicro: bigint): TransactionInstruction;
export declare function settlePosition(accounts: {
    owner: PublicKey;
    market: PublicKey;
    position: PublicKey;
    ownerTokenAccount: PublicKey;
    vault: PublicKey;
}): TransactionInstruction;
