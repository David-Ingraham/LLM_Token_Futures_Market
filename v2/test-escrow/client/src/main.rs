use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig,
        pubkey::Pubkey,
        signature::{read_keypair_file, Signer},
        system_program,
    },
    Client, Cluster,
};
use std::rc::Rc;
use std::thread;
use std::time::Duration;

const DEPOSIT_LAMPORTS: u64 = 1_000_000;

fn main() -> anyhow::Result<()> {
    let payer = read_keypair_file(format!(
        "{}/.config/solana/id.json",
        std::env::var("HOME")?
    ))
    .map_err(|e| anyhow::anyhow!("{e}"))?;

    let payer_pubkey = payer.pubkey();

    let client = Client::new_with_options(
        Cluster::Localnet,
        Rc::new(payer),
        CommitmentConfig::confirmed(),
    );

    let program = client.program(test_escrow::ID)?;

    let (escrow_pda, _) = Pubkey::find_program_address(
        &[test_escrow::ESCROW_SEED, payer_pubkey.as_ref()],
        &test_escrow::ID,
    );

    program
        .request()
        .accounts(test_escrow::accounts::Deposit {
            user: payer_pubkey,
            escrow: escrow_pda,
            system_program: system_program::ID,
        })
        .args(test_escrow::instruction::Deposit {
            amount: DEPOSIT_LAMPORTS,
        })
        .send()?;

    println!("deposited {DEPOSIT_LAMPORTS} lamports to escrow {escrow_pda}");
    println!(
        "waiting {} seconds before withdraw",
        test_escrow::LOCK_DURATION
    );

    thread::sleep(Duration::from_secs(test_escrow::LOCK_DURATION as u64));

    program
        .request()
        .accounts(test_escrow::accounts::Withdraw {
            depositor: payer_pubkey,
            escrow: escrow_pda,
        })
        .args(test_escrow::instruction::Withdraw {})
        .send()?;

    println!("withdrawn from escrow {escrow_pda}");
    Ok(())
}
