import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";
import {
  buildSnapshot,
  findOpus47PricingRow,
  parseOpus47OutputPriceMicro,
  PRICING_MD_URL,
} from "../src/parser.js";

const fixture = readFileSync(
  resolve(import.meta.dirname, "fixtures/pricing-snippet.md"),
  "utf8",
);

describe("oracle parser", () => {
  it("parses Opus 4.7 output column from markdown table row", () => {
    expect(parseOpus47OutputPriceMicro(fixture)).toBe(25_000_000);
  });

  it("finds the model row, not fast mode table", () => {
    const row = findOpus47PricingRow(fixture);
    expect(row).toContain("Claude Opus 4.7");
    expect(row).not.toContain("$150");
  });

  it("falls back to constant when row missing", () => {
    expect(parseOpus47OutputPriceMicro("no prices here")).toBe(25_000_000);
  });

  it("stable evidence hash when source row matches", () => {
    const url = PRICING_MD_URL;
    const a = buildSnapshot(fixture, url);
    const b = buildSnapshot(fixture, url);
    expect(a.evidenceHash).toBe(b.evidenceHash);
    expect(a.sourceRow).toContain("Opus 4.7");
  });
});
