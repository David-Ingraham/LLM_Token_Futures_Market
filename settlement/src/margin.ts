import { DEFAULT_CONTRACT_MTOK } from "./constants.js";
import type { MarketParams, Side } from "./types.js";
import { assertNonNegativeInt } from "./pnl.js";

/**
 * Minimum margin per position at open (micro-USDC).
 *
 * Long worst case: F_T = 0  -> loss = contracts * F_0 * contractMtok
 * Short worst case: F_T = max -> loss = contracts * (max - F_0) * contractMtok
 */
export function requiredMarginMicro(
  side: Side,
  contracts: number,
  entryPriceMicro: number,
  market: Pick<MarketParams, "maxSettlementPriceMicro" | "contractMtok">,
): number {
  const contractMtok = market.contractMtok ?? DEFAULT_CONTRACT_MTOK;
  assertNonNegativeInt(contracts, "contracts");
  assertNonNegativeInt(entryPriceMicro, "entryPriceMicro");
  assertNonNegativeInt(market.maxSettlementPriceMicro, "maxSettlementPriceMicro");

  if (entryPriceMicro > market.maxSettlementPriceMicro) {
    throw new Error("entry price above max settlement price");
  }

  if (side === "long") {
    return contracts * entryPriceMicro * contractMtok;
  }

  const upside = market.maxSettlementPriceMicro - entryPriceMicro;
  return contracts * upside * contractMtok;
}

/** True if locked margin covers worst-case loss for this side. */
export function isMarginSufficient(
  side: Side,
  contracts: number,
  entryPriceMicro: number,
  lockedMarginMicro: number,
  market: Pick<MarketParams, "maxSettlementPriceMicro" | "contractMtok">,
): boolean {
  return (
    lockedMarginMicro >=
    requiredMarginMicro(side, contracts, entryPriceMicro, market)
  );
}
