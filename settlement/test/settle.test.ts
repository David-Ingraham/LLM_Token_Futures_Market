import { describe, expect, it } from "vitest";
import { dollarsToMicro } from "../src/price.js";
import {
  settleMarket,
  totalPnlMicro,
  isVaultSolvent,
} from "../src/settle.js";
import { requiredMarginMicro } from "../src/margin.js";
import type { MarketParams, Position } from "../src/types.js";

const F0 = dollarsToMicro(25);
const FT = dollarsToMicro(27);

const market: MarketParams = {
  contractMtok: 1,
  maxSettlementPriceMicro: dollarsToMicro(50),
  feeBps: 0,
};

function openPosition(
  id: string,
  side: "long" | "short",
  contracts: number,
): Position {
  const margin = requiredMarginMicro(side, contracts, F0, market);
  return {
    id,
    side,
    contracts,
    entryPriceMicro: F0,
    lockedMarginMicro: margin,
  };
}

describe("settleMarket", () => {
  it("balanced long/short is zero-sum on pnl", () => {
    const positions = [
      openPosition("alice", "long", 10),
      openPosition("bob", "short", 10),
    ];
    const out = settleMarket(positions, FT, market);
    expect(totalPnlMicro(out.results)).toBe(0);
    expect(isVaultSolvent(out)).toBe(true);
    expect(out.vaultPaidOutMicro).toBe(out.vaultDepositedMicro);
  });

  it("long earns and short loses on price rise", () => {
    const out = settleMarket(
      [openPosition("alice", "long", 1), openPosition("bob", "short", 1)],
      FT,
      market,
    );
    const byId = Object.fromEntries(
      out.results.map((r) => [r.positionId, r]),
    );
    expect(byId.alice.pnlMicro).toBe(2_000_000);
    expect(byId.bob.pnlMicro).toBe(-2_000_000);
    expect(byId.alice.payoutMicro).toBe(
      openPosition("alice", "long", 1).lockedMarginMicro + 2_000_000,
    );
  });

  it("vault pays no more than was deposited when margins are correct", () => {
    const positions = [
      openPosition("a", "long", 3),
      openPosition("b", "short", 2),
      openPosition("c", "short", 1),
    ];
    const out = settleMarket(positions, FT, market);
    expect(isVaultSolvent(out)).toBe(true);
  });

  it("rejects settlement above max band", () => {
    expect(() =>
      settleMarket([openPosition("a", "long", 1)], dollarsToMicro(51), market),
    ).toThrow(/exceeds max/);
  });
});
