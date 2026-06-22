import { PublicKey } from "@solana/web3.js";
/** Deployed program id (devnet keypair in target/deploy). */
export declare const PROGRAM_ID: PublicKey;
export declare const MICRO_PER_UNIT = 1000000;
export declare const MARKET_MAGIC = 5569638132452967985n;
export declare const POSITION_MAGIC = 5786935714961966641n;
export declare const Side: {
    readonly Long: 0;
    readonly Short: 1;
};
export declare const MarketStatus: {
    readonly Open: 0;
    readonly Halted: 1;
    readonly PricePosted: 2;
};
