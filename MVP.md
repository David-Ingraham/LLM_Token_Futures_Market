# MVP — devnet walkthrough

## Units

| Path | Role |
|------|------|
| `settlement/` | Off-chain PnL/margin math (tests) |
| `programs/llm_token_futures/` | On-chain program (native Rust) |
| `packages/sdk/` | Instructions, PDAs, account decode |
| `oracle/` | Pricing snapshot + post-price CLI |
| `app/` | React UI (Phantom, devnet) |
| `scripts/deploy-devnet.ts` | Deploy program, mint test USDC, init market |

Program id: `9EBRakVzKCaHBUXdC8DMcUx9Seyf7o48eJS1wbN6ogpD`

## Build program

```bash
cd /path/to/llmTokenFutures
cargo-build-sbf --manifest-path programs/llm_token_futures/Cargo.toml --sbf-out-dir target/deploy
```

If lockfile errors appear, use Rust 1.75 to generate `Cargo.lock` and pin `blake3 = "=1.5.5"` in `programs/llm_token_futures/Cargo.toml`.

## Install and test

```bash
npm install
npm run test:settlement
npm run test -w @llm-token-futures/oracle
npm run build -w @llm-token-futures/sdk
```

## Deploy devnet

Requires Solana CLI on devnet and `~/.config/solana/id.json` funded.

```bash
solana config set --url devnet
npm run deploy:devnet
npm run dev
```

Open http://localhost:5173 — connect Phantom on **devnet**.

## Full process in UI

1. **Open position** — long or short; USDC margin moves to vault.
2. **Halt trading** — after `trade_cutoff` (deploy sets +1h).
3. **Post settlement price** — oracle wallet (same as deploy authority by default); or CLI below.
4. **Settle position** — USDC payout to your ATA.

## Oracle CLI

Fetches Anthropic pricing markdown: `https://platform.claude.com/docs/en/about-claude/pricing.md`

```bash
npm run oracle:snapshot
MARKET_PUBKEY=<from devnet.json> npm run oracle:post
```

Optional: `SNAPSHOT_FILE=oracle/snapshots/snapshot-....json`

## Two-wallet demo

Use a second devnet wallet as short while first is long: import another keypair in Phantom, open opposite side before halt.
