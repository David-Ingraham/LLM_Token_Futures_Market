import { describe, expect, it } from "vitest";
import { dollarsToMicro } from "../src/price.js";
import { requiredMarginMicro, isMarginSufficient } from "../src/margin.js";
import type { MarketParams } from "../src/types.js";

const market: MarketParams = {
  contractMtok: 1,
  maxSettlementPriceMicro: dollarsToMicro(50),
  feeBps: 0,
};

const F0 = dollarsToMicro(25);

describe("requiredMarginMicro", () => {
  it("long requires contracts * entry (worst case price -> 0)", () => {
    expect(requiredMarginMicro("long", 10, F0, market)).toBe(250_000_000);
  });

  it("short requires contracts * (max - entry)", () => {
    expect(requiredMarginMicro("short", 10, F0, market)).toBe(250_000_000);
  });
});

describe("isMarginSufficient", () => {
  it("accepts exact required margin", () => {
    const m = requiredMarginMicro("long", 4, F0, market);
    expect(isMarginSufficient("long", 4, F0, m, market)).toBe(true);
  });

  it("rejects undercollateralized", () => {
    expect(isMarginSufficient("long", 4, F0, 1, market)).toBe(false);
  });
});
