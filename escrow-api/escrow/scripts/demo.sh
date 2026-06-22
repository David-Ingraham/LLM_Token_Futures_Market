#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."

echo "Deploying to devnet..."
anchor deploy --provider.cluster devnet

echo "Depositing 0.01 SOL..."
npx ts-node scripts/interact.ts deposit 0.01

echo "Escrow status:"
npx ts-node scripts/interact.ts status

echo "After 5 minutes run: npx ts-node scripts/interact.ts release"
