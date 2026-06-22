import {
  buildSnapshot,
  OPUS_47_OUTPUT_USD_PER_MTOK,
  PRICING_MD_URL,
  type PriceSnapshot,
} from "./parser.js";

export async function fetchPricingSnapshot(
  sourceUrl = PRICING_MD_URL,
): Promise<PriceSnapshot> {
  try {
    const res = await fetch(sourceUrl, {
      headers: {
        "User-Agent": "llm-token-futures-oracle/0.1",
        Accept: "text/markdown,text/plain,*/*",
      },
    });
    if (!res.ok) {
      return fallbackSnapshot(sourceUrl, `http ${res.status}`);
    }
    const body = await res.text();
    return buildSnapshot(body, sourceUrl);
  } catch (err) {
    const msg = err instanceof Error ? err.message : String(err);
    return fallbackSnapshot(sourceUrl, msg);
  }
}

function fallbackSnapshot(sourceUrl: string, reason: string): PriceSnapshot {
  const row =
    "| Claude Opus 4.7 | $5 / MTok | $6.25 / MTok | $10 / MTok | $0.50 / MTok | $25 / MTok |";
  const body = `${row}\n(fallback: ${reason})`;
  const snap = buildSnapshot(body, sourceUrl);
  return {
    ...snap,
    sourceUrl: `${sourceUrl}#fallback`,
    outputUsdPerMtok: OPUS_47_OUTPUT_USD_PER_MTOK,
    outputPriceMicro: Math.round(OPUS_47_OUTPUT_USD_PER_MTOK * 1_000_000),
  };
}
