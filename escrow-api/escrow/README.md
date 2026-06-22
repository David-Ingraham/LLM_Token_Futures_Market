# Escrow (Seahorse)

Timed SOL escrow on devnet. Deposit via the program, wait 5 minutes, call `release` to get funds back.

Program logic: `programs_py/escrow.py` (single file).

## Build

```bash
cd escrow-api/escrow
seahorse build
anchor build
```

## Deploy (devnet)

Uses your default Solana CLI wallet (`~/.config/solana/id.json`). Program deploy needs ~1.6 SOL on devnet.

```bash
solana balance --url devnet
solana airdrop 2 --url devnet
anchor deploy --provider.cluster devnet
```

Program id: `JBSsJbhEHwBBy3NC9fxw33Up4PaTPHXHKoVYME7RxeSb`

## Interact

`solana transfer` to the vault PDA does not start the timer. Call `deposit`:

```bash
npx ts-node scripts/interact.ts deposit 0.01
npx ts-node scripts/interact.ts status
npx ts-node scripts/interact.ts release   # after 5 minutes
```

Vault PDA seeds: `["escrow", your_wallet_pubkey]`.
