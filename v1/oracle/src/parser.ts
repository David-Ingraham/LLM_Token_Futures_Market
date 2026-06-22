import { createHash } from "node:crypto";

const MICRO_PER_UNIT = 1_000_000;

/** Official docs markdown (model pricing table). */
export const PRICING_MD_URL =
  "https://platform.claude.com/docs/en/about-claude/pricing.md";

/** Opus 4.7 output column default if row missing. */
export const OPUS_47_OUTPUT_USD_PER_MTOK = 25;

/** Model pricing table: last column is "Output Tokens". */
const OUTPUT_COLUMN_INDEX = 5;

export type PriceSnapshot = {
  modelId: string;
  outputUsdPerMtok: number;
  outputPriceMicro: number;
  observedAt: string;
  sourceUrl: string;
  evidenceHash: string;
  /** Matched markdown table row, if any */
  sourceRow: string | null;
};

/**
 * Parse Opus 4.7 output $/MTok from Anthropic pricing markdown table row.
 */
export function parseOpus47OutputPriceMicro(markdown: string): number {
  const row = findOpus47PricingRow(markdown);
  if (!row) {
    return Math.round(OPUS_47_OUTPUT_USD_PER_MTOK * MICRO_PER_UNIT);
  }

  const cells = splitTableRow(row);
  if (cells.length <= OUTPUT_COLUMN_INDEX) {
    return Math.round(OPUS_47_OUTPUT_USD_PER_MTOK * MICRO_PER_UNIT);
  }

  const outputCell = cells[OUTPUT_COLUMN_INDEX];
  const usd = parseUsdPerMtokCell(outputCell);
  if (usd == null) {
    return Math.round(OPUS_47_OUTPUT_USD_PER_MTOK * MICRO_PER_UNIT);
  }

  return Math.round(usd * MICRO_PER_UNIT);
}

export function findOpus47PricingRow(markdown: string): string | null {
  for (const line of markdown.split("\n")) {
    const trimmed = line.trim();
    if (!/^\|/.test(trimmed)) continue;
    if (!/\bClaude Opus 4\.7\b/i.test(trimmed)) continue;
    if (/^\|[\s\-:|]+\|$/i.test(trimmed)) continue;
    const cells = splitTableRow(trimmed);
    if (cells.length >= 6 && /^claude opus 4\.7$/i.test(cells[0])) {
      return trimmed;
    }
  }
  return null;
}

export function splitTableRow(row: string): string[] {
  return row
    .split("|")
    .map((c) => c.trim())
    .filter((c) => c.length > 0);
}

export function parseUsdPerMtokCell(cell: string): number | null {
  const m = cell.match(/\$\s*([\d.]+)\s*\/\s*MTok/i);
  if (!m) return null;
  const usd = parseFloat(m[1]);
  return Number.isFinite(usd) && usd > 0 ? usd : null;
}

export function buildSnapshot(
  body: string,
  sourceUrl: string,
): PriceSnapshot {
  const sourceRow = findOpus47PricingRow(body);
  const outputPriceMicro = parseOpus47OutputPriceMicro(body);
  const canonical = JSON.stringify({
    modelId: "claude-opus-4-7",
    outputPriceMicro,
    sourceUrl,
    sourceRow,
  });
  const evidenceHash = createHash("sha256").update(canonical).digest("hex");

  return {
    modelId: "claude-opus-4-7",
    outputUsdPerMtok: outputPriceMicro / MICRO_PER_UNIT,
    outputPriceMicro,
    observedAt: new Date().toISOString(),
    sourceUrl,
    evidenceHash,
    sourceRow,
  };
}
