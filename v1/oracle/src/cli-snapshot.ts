import { writeFileSync, mkdirSync } from "node:fs";
import { resolve } from "node:path";
import { fetchPricingSnapshot } from "./fetch.js";

const outDir = resolve(process.cwd(), "oracle/snapshots");
mkdirSync(outDir, { recursive: true });

const snap = await fetchPricingSnapshot();
const file = resolve(outDir, `snapshot-${Date.now()}.json`);
writeFileSync(file, JSON.stringify(snap, null, 2));

console.log("Oracle snapshot written:", file);
console.log(JSON.stringify(snap, null, 2));
