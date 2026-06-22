/**
 * Run: npm run demo
 * Prints one balanced market settlement at $25 -> $27.
 */
import { dollarsToMicro, formatMicroUsd } from "../src/price.js";
import { requiredMarginMicro } from "../src/margin.js";
import { settleMarket } from "../src/settle.js";
import type { MarketParams, Position } from "../src/types.js";

const market: MarketParams = {
  contractMtok: 1,
  maxSettlementPriceMicro: dollarsToMicro(50),
  feeBps: 0,
};

const entry = dollarsToMicro(25);
const settlement = dollarsToMicro(27);

function make(id: string, side: "long" | "short", contracts: number): Position {
  const lockedMarginMicro = requiredMarginMicro(
    side,
    contracts,
    entry,
    market,
  );
  return { id, side, contracts, entryPriceMicro: entry, lockedMarginMicro };
}

const positions = [make("long-user", "long", 10), make("short-user", "short", 10)];
const out = settleMarket(positions, settlement, market);

console.log("Opus 4.7 output index futures — settlement demo\n");
console.log(`Entry F0:     ${formatMicroUsd(entry)} / MTok`);
console.log(`Settlement FT: ${formatMicroUsd(settlement)} / MTok\n`);

for (const r of out.results) {
  const p = positions.find((x) => x.id === r.positionId)!;
  console.log(`${r.positionId} (${p.side}, ${p.contracts} contracts)`);
  console.log(`  margin in:  ${formatMicroUsd(p.lockedMarginMicro)}`);
  console.log(`  PnL:        ${formatMicroUsd(r.pnlMicro)}`);
  console.log(`  payout:     ${formatMicroUsd(r.payoutMicro)}\n`);
}

console.log(`Vault deposited: ${formatMicroUsd(out.vaultDepositedMicro)}`);
console.log(`Vault paid out:  ${formatMicroUsd(out.vaultPaidOutMicro)}`);
