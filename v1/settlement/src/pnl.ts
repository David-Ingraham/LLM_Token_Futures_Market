import { DEFAULT_CONTRACT_MTOK } from "./constants.js";
import type { MarketParams, Side } from "./types.js";

/**
 * PnL in micro-USDC for one position at settlement.
 *
 * Long:  contracts * (F_T - F_0) * contractMtok
 * Short: contracts * (F_0 - F_T) * contractMtok
 *
 * With contractMtok = 1, one contract moving $1/MTok pays $1 per contract.
 */
export function settlementPnlMicro(
  side: Side,
  contracts: number,
  entryPriceMicro: number,
  settlementPriceMicro: number,
  contractMtok: number = DEFAULT_CONTRACT_MTOK,
): number {
  assertNonNegativeInt(contracts, "contracts");
  assertNonNegativeInt(entryPriceMicro, "entryPriceMicro");
  assertNonNegativeInt(settlementPriceMicro, "settlementPriceMicro");
  assertNonNegativeInt(contractMtok, "contractMtok");

  const priceDelta = settlementPriceMicro - entryPriceMicro;
  const raw = contracts * priceDelta * contractMtok;
  const signed = side === "long" ? raw : -raw;

  return signed === 0 ? 0 : signed;
}

export function settlementFeeMicro(
  pnlMicro: number,
  feeBps: number,
): number {
  if (feeBps < 0 || feeBps > 10_000) {
    throw new Error(`invalid feeBps: ${feeBps}`);
  }
  if (feeBps === 0 || pnlMicro === 0) {
    return 0;
  }
  const absPnl = Math.abs(pnlMicro);
  return Math.floor((absPnl * feeBps) / 10_000);
}

/** Payout = margin + pnl - fee (fee only charged on positive pnl in v1; see README optional fees) */
export function settlementPayoutMicro(
  lockedMarginMicro: number,
  pnlMicro: number,
  feeMicro: number,
): number {
  assertNonNegativeInt(lockedMarginMicro, "lockedMarginMicro");
  const payout = lockedMarginMicro + pnlMicro - feeMicro;
  if (payout < 0) {
    throw new Error(
      `payout negative: margin=${lockedMarginMicro} pnl=${pnlMicro} fee=${feeMicro}`,
    );
  }
  return payout;
}

export function assertSettlementPriceInBand(
  settlementPriceMicro: number,
  maxSettlementPriceMicro: number,
): void {
  assertNonNegativeInt(settlementPriceMicro, "settlementPriceMicro");
  assertNonNegativeInt(maxSettlementPriceMicro, "maxSettlementPriceMicro");
  if (settlementPriceMicro > maxSettlementPriceMicro) {
    throw new Error(
      `settlement ${settlementPriceMicro} exceeds max ${maxSettlementPriceMicro}`,
    );
  }
}

export function assertNonNegativeInt(value: number, name: string): void {
  if (!Number.isSafeInteger(value) || value < 0) {
    throw new Error(`invalid ${name}: ${value}`);
  }
}
