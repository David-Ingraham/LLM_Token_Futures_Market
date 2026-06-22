import type {
  MarketParams,
  MarketSettlement,
  Position,
  SettlementResult,
} from "./types.js";
import {
  assertSettlementPriceInBand,
  settlementFeeMicro,
  settlementPayoutMicro,
  settlementPnlMicro,
} from "./pnl.js";
import { isMarginSufficient } from "./margin.js";

export function settlePosition(
  position: Position,
  settlementPriceMicro: number,
  market: MarketParams,
): SettlementResult {
  if (!isMarginSufficient(
    position.side,
    position.contracts,
    position.entryPriceMicro,
    position.lockedMarginMicro,
    market,
  )) {
    throw new Error(`insufficient margin for position ${position.id}`);
  }

  const pnlMicro = settlementPnlMicro(
    position.side,
    position.contracts,
    position.entryPriceMicro,
    settlementPriceMicro,
    market.contractMtok,
  );

  const feeMicro = settlementFeeMicro(pnlMicro, market.feeBps);
  const payoutMicro = settlementPayoutMicro(
    position.lockedMarginMicro,
    pnlMicro,
    feeMicro,
  );

  return {
    positionId: position.id,
    pnlMicro,
    feeMicro,
    payoutMicro,
  };
}

/** Settle every position in a market; check vault accounting. */
export function settleMarket(
  positions: Position[],
  settlementPriceMicro: number,
  market: MarketParams,
): MarketSettlement {
  assertSettlementPriceInBand(
    settlementPriceMicro,
    market.maxSettlementPriceMicro,
  );

  const results = positions.map((p) =>
    settlePosition(p, settlementPriceMicro, market),
  );

  const vaultDepositedMicro = positions.reduce(
    (sum, p) => sum + p.lockedMarginMicro,
    0,
  );
  const vaultPaidOutMicro = results.reduce(
    (sum, r) => sum + r.payoutMicro,
    0,
  );

  return {
    settlementPriceMicro,
    results,
    vaultDepositedMicro,
    vaultPaidOutMicro,
  };
}

/**
 * Zero-sum check: total PnL across positions should be 0 when book is balanced.
 * Vault: payouts equal deposits + sum(pnl) - sum(fees). Fees stay in vault (not modeled to treasury here).
 */
export function totalPnlMicro(results: SettlementResult[]): number {
  return results.reduce((sum, r) => sum + r.pnlMicro, 0);
}

export function isVaultSolvent(settlement: MarketSettlement): boolean {
  return settlement.vaultPaidOutMicro <= settlement.vaultDepositedMicro;
}
