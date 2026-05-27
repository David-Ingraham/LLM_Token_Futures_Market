# Settlement math (v1)

Reference implementation of MVP settlement formulas. Same integer units planned for Solana (micro-dollars per MTok, micro-USDC).

## Modules

| File | MVP role |
|------|----------|
| `src/price.ts` | Convert $/MTok ↔ on-chain integers |
| `src/pnl.ts` | PnL, fees, payout for one position |
| `src/margin.ts` | Minimum margin at open |
| `src/settle.ts` | Settle a whole market; vault solvency check |
| `src/types.ts` | Position / market shapes (mirror ledger later) |

## Commands

```bash
cd settlement
npm install
npm test
npm run demo
```

Later: import these functions from Anchor/TypeScript client tests, or port formulas 1:1 to Rust in the program.
