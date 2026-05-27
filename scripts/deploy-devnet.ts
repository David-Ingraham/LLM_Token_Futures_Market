import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
} from "@solana/spl-token";
import {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  sendAndConfirmTransaction,
  Transaction,
} from "@solana/web3.js";
import { execSync } from "node:child_process";
import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { resolve } from "node:path";
import {
  initializeMarket,
  marketPda,
  vaultPda,
  PROGRAM_ID,
} from "../packages/sdk/src/index.ts";

const RPC = process.env.SOLANA_RPC ?? "https://api.devnet.solana.com";
const KEYPAIR_PATH =
  process.env.SOLANA_KEYPAIR ?? `${process.env.HOME}/.config/solana/id.json`;

const connection = new Connection(RPC, "confirmed");
const payer = loadKeypair(KEYPAIR_PATH);
const programKeypairPath = resolve("target/deploy/llm_token_futures-keypair.json");

async function main() {
  const balance = await connection.getBalance(payer.publicKey);
  if (balance < 0.5 * LAMPORTS_PER_SOL) {
    console.log("Airdropping 2 SOL on devnet...");
    const sig = await connection.requestAirdrop(
      payer.publicKey,
      2 * LAMPORTS_PER_SOL,
    );
    await connection.confirmTransaction(sig, "confirmed");
  }

  const programPath = resolve("target/deploy/llm_token_futures.so");
  const existing = await connection.getAccountInfo(PROGRAM_ID);

  if (!existing?.executable) {
    console.log("Deploying program:", PROGRAM_ID.toBase58());
    execSync(
      `solana program deploy "${programPath}" --program-id "${programKeypairPath}" --url devnet`,
      { stdio: "inherit" },
    );
  } else {
    console.log("Program already on devnet");
  }

  const mint = await createMint(connection, payer, payer.publicKey, null, 6);
  const ata = await getOrCreateAssociatedTokenAccount(
    connection,
    payer,
    mint,
    payer.publicKey,
  );
  await mintTo(connection, payer, mint, ata.address, payer, 1_000_000_000_000);

  const marketId = BigInt(Math.floor(Date.now() / 1000));
  const now = BigInt(Math.floor(Date.now() / 1000));
  const [market] = marketPda(payer.publicKey, marketId);
  const [vault] = vaultPda(market);

  const initIx = initializeMarket(
    {
      authority: payer.publicKey,
      oracle: payer.publicKey,
      mint,
      market,
      vault,
    },
    {
      marketId,
      entryPriceMicro: 25_000_000n,
      maxSettlementPriceMicro: 50_000_000n,
      openTs: now - 60n,
      tradeCutoffTs: now + 3600n,
      expiryTs: now + 7200n,
      feeBps: 0,
    },
  );

  const initSig = await sendAndConfirmTransaction(
    connection,
    new Transaction().add(initIx),
    [payer],
  );

  const config = {
    cluster: "devnet",
    rpc: RPC,
    programId: PROGRAM_ID.toBase58(),
    mint: mint.toBase58(),
    market: market.toBase58(),
    vault: vault.toBase58(),
    authority: payer.publicKey.toBase58(),
    oracle: payer.publicKey.toBase58(),
    marketId: marketId.toString(),
    entryPriceMicro: "25000000",
    tradeCutoffTs: (now + 3600n).toString(),
    expiryTs: (now + 7200n).toString(),
  };

  mkdirSync("app/public", { recursive: true });
  writeFileSync("app/public/devnet.json", JSON.stringify(config, null, 2));
  writeFileSync("devnet.json", JSON.stringify(config, null, 2));
  console.log("Market:", market.toBase58());
  console.log("Init sig:", initSig);
  console.log("Wrote app/public/devnet.json — run: npm run dev");
}

function loadKeypair(path: string): Keypair {
  const raw = JSON.parse(readFileSync(path, "utf8"));
  return Keypair.fromSecretKey(Uint8Array.from(raw));
}

main().catch((e) => {
  console.error(e);
  process.exit(1);
});
