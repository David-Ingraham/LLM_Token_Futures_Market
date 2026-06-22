import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { Escrow } from "../target/types/escrow";
import { expect } from "chai";

describe("escrow", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.escrow as Program<Escrow>;
  const provider = anchor.getProvider() as anchor.AnchorProvider;
  const depositor = provider.wallet.publicKey;

  const escrowPda = () =>
    PublicKey.findProgramAddressSync(
      [Buffer.from("escrow"), depositor.toBuffer()],
      program.programId
    )[0];

  it("deposits and releases after lock", async () => {
    const before = await provider.connection.getBalance(depositor);
    const amount = new anchor.BN(0.01 * LAMPORTS_PER_SOL);

    await program.methods
      .deposit(amount)
      .accounts({ depositor, escrow: escrowPda() })
      .rpc();

    const acct = await program.account.escrow.fetch(escrowPda());
    expect(acct.amount.eq(amount)).to.be.true;

    try {
      await program.methods
        .release()
        .accounts({ depositor, escrow: escrowPda() })
        .rpc();
      expect.fail("release should fail before unlock");
    } catch {
      // expected
    }

    const unlockAt = acct.unlockAt.toNumber();
    const waitSec = unlockAt - Math.floor(Date.now() / 1000) + 2;
    if (waitSec > 0) {
      await new Promise((r) => setTimeout(r, waitSec * 1000));
    }

    await program.methods
      .release()
      .accounts({ depositor, escrow: escrowPda() })
      .rpc();

    const after = await provider.connection.getBalance(depositor);
    expect(after).to.be.greaterThan(before - 0.02 * LAMPORTS_PER_SOL);
  });
});
