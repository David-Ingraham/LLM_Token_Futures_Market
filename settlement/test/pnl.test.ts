import { describe, expect, it } from "vitest";
import { dollarsToMicro } from "../src/price.js";
import {
  settlementFeeMicro,
  settlementPayoutMicro,
  settlementPnlMicro,
} from "../src/pnl.js";

const F0 = dollarsToMicro(25);
const F_UP = dollarsToMicro(27);
const F_DOWN = dollarsToMicro(23);
const F_FLAT = F0;

describe("settlementPnlMicro", () => {
  it("long gains when settlement rises", () => {
    expect(settlementPnlMicro("long", 10, F0, F_UP)).toBe(20_000_000);
  });

  it("long loses when settlement falls", () => {
    expect(settlementPnlMicro("long", 10, F0, F_DOWN)).toBe(-20_000_000);
  });

  it("short gains when settlement falls", () => {
    expect(settlementPnlMicro("short", 10, F0, F_DOWN)).toBe(20_000_000);
  });

  it("flat settlement has zero pnl", () => {
    expect(settlementPnlMicro("long", 5, F_FLAT, F_FLAT)).toBe(0);
    expect(settlementPnlMicro("short", 5, F_FLAT, F_FLAT)).toBe(0);
  });

  it("scales with contract count", () => {
    expect(settlementPnlMicro("long", 1, F0, F_UP)).toBe(2_000_000);
    expect(settlementPnlMicro("long", 100, F0, F_UP)).toBe(200_000_000);
  });
});

describe("settlementPayoutMicro", () => {
  it("returns margin plus pnl minus fee", () => {
    const margin = 250_000_000;
    const pnl = 20_000_000;
    const fee = settlementFeeMicro(pnl, 50);
    expect(settlementPayoutMicro(margin, pnl, fee)).toBe(
      margin + pnl - fee,
    );
  });

  it("rejects payout below zero", () => {
    expect(() =>
      settlementPayoutMicro(1_000_000, -5_000_000, 0),
    ).toThrow(/payout negative/);
  });
});
