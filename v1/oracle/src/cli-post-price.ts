import { readFileSync } from "node:fs";
import { Connection, Keypair, PublicKey, sendAndConfirmTransaction, Transaction } from "@solana/web3.js";
import { postSettlementPrice } from "@llm-token-futures/sdk";
import { fetchPricingSnapshot } from "./fetch.js";

const RPC = process.env.SOLANA_RPC ?? "https://api.devnet.solana.com";
const MARKET = process.env.MARKET_PUBKEY;
const ORACLE_KEYPAIR = process.env.ORACLE_KEYPAIR ?? process.env.SOLANA_KEYPAIR;
const SNAPSHOT_FILE = process.env.SNAPSHOT_FILE;

if (!MARKET) {
  console.error("Set MARKET_PUBKEY to the market account address");
  process.exit(1);
}

const connection = new Connection(RPC, "confirmed");
const oracle = loadKeypair(ORACLE_KEYPAIR!);
const market = new PublicKey(MARKET);

let priceMicro: bigint;
if (SNAPSHOT_FILE) {
  const snap = JSON.parse(readFileSync(SNAPSHOT_FILE, "utf8"));
  priceMicro = BigInt(snap.outputPriceMicro);
} else {
  const snap = await fetchPricingSnapshot();
  priceMicro = BigInt(snap.outputPriceMicro);
  console.log("Snapshot:", snap);
}

const ix = postSettlementPrice(oracle.publicKey, market, priceMicro);
const tx = new Transaction().add(ix);
const sig = await sendAndConfirmTransaction(connection, tx, [oracle]);
console.log("Posted settlement price:", priceMicro.toString(), "sig:", sig);

function loadKeypair(pathOrJson: string): Keypair {
  if (pathOrJson.trim().startsWith("[")) {
    return Keypair.fromSecretKey(Uint8Array.from(JSON.parse(pathOrJson)));
  }
  const raw = JSON.parse(readFileSync(pathOrJson.replace("~", process.env.HOME ?? ""), "utf8"));
  return Keypair.fromSecretKey(Uint8Array.from(raw));
}
