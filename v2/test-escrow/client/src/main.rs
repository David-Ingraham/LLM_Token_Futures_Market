use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig,
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair, Signer},
        system_program,
    },
    Client, Cluster,
};
use std::rc::Rc;
use std::thread;
use std::time::Duration;

const DEPOSIT_LAMPORTS: u64 = 1_000;
const WALLET1_PATH: &str = ".config/solana/id.json";
const WALLET2_PATH: &str = ".config/solana/wallet2.json";

fn load_keypair(home: &str, path: &str) -> anyhow::Result<Keypair> {
    read_keypair_file(format!("{home}/{path}")).map_err(|e| anyhow::anyhow!("{e}"))
}

fn deposit(
    client: &Client<Rc<Keypair>>,
    user: &Rc<Keypair>,
    pool_pda: Pubkey,
) -> anyhow::Result<()> {
    let program = client.program(test_escrow::ID)?;
    let user_pubkey = user.pubkey();
    let (user_deposit_pda, _) = Pubkey::find_program_address(
        &[test_escrow::USER_DEPOSIT_SEED, user_pubkey.as_ref()],
        &test_escrow::ID,
    );

    program
        .request()
        .accounts(test_escrow::accounts::Deposit {
            user: user_pubkey,
            pool: pool_pda,
            user_deposit: user_deposit_pda,
            system_program: system_program::ID,
        })
        .args(test_escrow::instruction::Deposit {
            amount: DEPOSIT_LAMPORTS,
        })
        .send()?;

    println!(
        "deposited {DEPOSIT_LAMPORTS} lamports from {} to pool {pool_pda}",
        user_pubkey
    );
    Ok(())
}

fn withdraw(
    client: &Client<Rc<Keypair>>,
    user: &Rc<Keypair>,
    pool_pda: Pubkey,
) -> anyhow::Result<()> {
    let program = client.program(test_escrow::ID)?;
    let user_pubkey = user.pubkey();
    let (user_deposit_pda, _) = Pubkey::find_program_address(
        &[test_escrow::USER_DEPOSIT_SEED, user_pubkey.as_ref()],
        &test_escrow::ID,
    );

    program
        .request()
        .accounts(test_escrow::accounts::Withdraw {
            depositor: user_pubkey,
            pool: pool_pda,
            user_deposit: user_deposit_pda,
        })
        .args(test_escrow::instruction::Withdraw {})
        .send()?;

    println!(
        "withdrew {DEPOSIT_LAMPORTS} lamports to {} from pool {pool_pda}",
        user_pubkey
    );
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let home = std::env::var("HOME")?;
    let wallet1 = Rc::new(load_keypair(&home, WALLET1_PATH)?);
    let wallet2 = Rc::new(load_keypair(&home, WALLET2_PATH)?);

    let client1 = Client::new_with_options(
        Cluster::Localnet,
        Rc::clone(&wallet1),
        CommitmentConfig::confirmed(),
    );
    let client2 = Client::new_with_options(
        Cluster::Localnet,
        Rc::clone(&wallet2),
        CommitmentConfig::confirmed(),
    );

    let (pool_pda, _) =
        Pubkey::find_program_address(&[test_escrow::POOL_SEED], &test_escrow::ID);

    deposit(&client1, &wallet1, pool_pda)?;
    deposit(&client2, &wallet2, pool_pda)?;

    println!(
        "waiting {} seconds before withdraw",
        test_escrow::LOCK_DURATION
    );
    thread::sleep(Duration::from_secs(test_escrow::LOCK_DURATION as u64));

    withdraw(&client1, &wallet1, pool_pda)?;
    withdraw(&client2, &wallet2, pool_pda)?;

    Ok(())
}
