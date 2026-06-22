import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { Escrow } from "../target/types/escrow";

async function main() {
  const cmd = process.argv[2] ?? "status";
  const arg = process.argv[3];

  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider() as anchor.AnchorProvider;
  const program = anchor.workspace.escrow as Program<Escrow>;
  const depositor = provider.wallet.publicKey;

  const [escrowPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("escrow"), depositor.toBuffer()],
    program.programId
  );

  if (cmd === "deposit") {
    const sol = parseFloat(arg ?? "0.01");
    const lamports = new anchor.BN(Math.round(sol * LAMPORTS_PER_SOL));
    const sig = await program.methods
      .deposit(lamports)
      .accounts({
        depositor,
        escrow: escrowPda,
      })
      .rpc();
    console.log("deposit tx:", sig);
    console.log("escrow pda:", escrowPda.toBase58());
    return;
  }

  if (cmd === "release") {
    const sig = await program.methods
      .release()
      .accounts({
        depositor,
        escrow: escrowPda,
      })
      .rpc();
    console.log("release tx:", sig);
    return;
  }

  const acct = await program.account.escrow.fetchNullable(escrowPda);
  if (!acct) {
    console.log("no escrow for", depositor.toBase58());
    console.log("pda would be", escrowPda.toBase58());
    return;
  }

  const bal = await provider.connection.getBalance(escrowPda);
  const now = Math.floor(Date.now() / 1000);
  const unlockAt = acct.unlockAt.toNumber();
  console.log({
    program: program.programId.toBase58(),
    escrowPda: escrowPda.toBase58(),
    owner: acct.owner.toBase58(),
    amountLamports: acct.amount.toString(),
    unlockAt,
    secondsLeft: Math.max(0, unlockAt - now),
    vaultBalanceLamports: bal,
  });
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
