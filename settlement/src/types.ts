/** Long profits when settlement price rises; short profits when it falls. */
export type Side = "long" | "short";

/**
 * Open position before expiry. Amounts are integers in micro-units (see constants.ts).
 */
export type Position = {
  id: string;
  side: Side;
  contracts: number;
  /** F0: entry index, micro-dollars per MTok */
  entryPriceMicro: number;
  /** USDC locked at open, micro-USDC (6 decimals, same scale as price PnL) */
  lockedMarginMicro: number;
};

export type MarketParams = {
  /** v1: 1 contract = 1 MTok of index exposure */
  contractMtok: number;
  /** Oracle ceiling for index at settlement; used for short margin */
  maxSettlementPriceMicro: number;
  /** Optional fee in basis points on absolute PnL at settlement (0 = none) */
  feeBps: number;
};

export type SettlementResult = {
  positionId: string;
  pnlMicro: number;
  feeMicro: number;
  payoutMicro: number;
};

export type MarketSettlement = {
  settlementPriceMicro: number;
  results: SettlementResult[];
  vaultDepositedMicro: number;
  vaultPaidOutMicro: number;
};
