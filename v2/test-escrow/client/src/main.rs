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
use test_escrow::{Market, Position, Side, MARKET_SEED, POSITION_SEED};

const WALLET1_PATH: &str = ".config/solana/id.json";
const WALLET2_PATH: &str = ".config/solana/wallet2.json";

const MARGIN_PER_CONTRACT: u64 = 25;
const ENTRY_PRICE: u64 = 25;
const SETTLEMENT_PRICE: u64 = 30;
const QTY: u64 = 1;
const EXPIRY_OFFSET_SECS: i64 = 5;

fn load_keypair(home: &str, path: &str) -> anyhow::Result<Keypair> {
    read_keypair_file(format!("{home}/{path}")).map_err(|e| anyhow::anyhow!("{e}"))
}

fn derive_market_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[MARKET_SEED], &test_escrow::ID)
}

fn derive_position_pda(market: &Pubkey, user: &Pubkey, side: Side) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            POSITION_SEED,
            market.as_ref(),
            user.as_ref(),
            &[side.as_seed_byte()],
        ],
        &test_escrow::ID,
    )
}

fn fetch_market(client: &Client<Rc<Keypair>>, market: Pubkey) -> anyhow::Result<Market> {
    let program = client.program(test_escrow::ID)?;
    Ok(program.account::<Market>(market)?)
}

fn fetch_position(client: &Client<Rc<Keypair>>, position: Pubkey) -> anyhow::Result<Position> {
    let program = client.program(test_escrow::ID)?;
    Ok(program.account::<Position>(position)?)
}

fn initialize_market(
    client: &Client<Rc<Keypair>>,
    authority: &Rc<Keypair>,
    market: Pubkey,
    expiry_ts: i64,
) -> anyhow::Result<()> {
    let program = client.program(test_escrow::ID)?;
    program
        .request()
        .accounts(test_escrow::accounts::InitializeMarket {
            authority: authority.pubkey(),
            market,
            system_program: system_program::ID,
        })
        .args(test_escrow::instruction::InitializeMarket {
            expiry_ts,
            margin_per_contract: MARGIN_PER_CONTRACT,
        })
        .send()?;
    println!("market initialized at {market}");
    Ok(())
}

fn open_position(
    client: &Client<Rc<Keypair>>,
    user: &Rc<Keypair>,
    market: Pubkey,
    side: Side,
    qty: u64,
    entry_price: u64,
) -> anyhow::Result<()> {
    let program = client.program(test_escrow::ID)?;
    let user_pubkey = user.pubkey();
    let (position, _) = derive_position_pda(&market, &user_pubkey, side);

    program
        .request()
        .accounts(test_escrow::accounts::OpenPosition {
            user: user_pubkey,
            market,
            position,
            system_program: system_program::ID,
        })
        .args(test_escrow::instruction::OpenPosition {
            side,
            qty,
            entry_price,
        })
        .send()?;

    let label = match side {
        Side::Long => "long",
        Side::Short => "short",
    };
    println!("{user_pubkey} opened {qty} {label} at entry {entry_price}");
    Ok(())
}

fn set_settlement_price(
    client: &Client<Rc<Keypair>>,
    authority: &Rc<Keypair>,
    market: Pubkey,
    settlement_price: u64,
) -> anyhow::Result<()> {
    let program = client.program(test_escrow::ID)?;
    program
        .request()
        .accounts(test_escrow::accounts::SetSettlementPrice {
            authority: authority.pubkey(),
            market,
        })
        .args(test_escrow::instruction::SetSettlementPrice { settlement_price })
        .send()?;
    println!("settlement price set to {settlement_price}");
    Ok(())
}

fn claim(
    client: &Client<Rc<Keypair>>,
    user: &Rc<Keypair>,
    market: Pubkey,
    side: Side,
) -> anyhow::Result<()> {
    let program = client.program(test_escrow::ID)?;
    let user_pubkey = user.pubkey();
    let (position, _) = derive_position_pda(&market, &user_pubkey, side);

    program
        .request()
        .accounts(test_escrow::accounts::Claim {
            user: user_pubkey,
            market,
            position,
        })
        .args(test_escrow::instruction::Claim { side })
        .send()?;

    let label = match side {
        Side::Long => "long",
        Side::Short => "short",
    };
    println!("{user_pubkey} claimed {label} position");
    Ok(())
}

fn print_market(client: &Client<Rc<Keypair>>, market: Pubkey) -> anyhow::Result<()> {
    let m = fetch_market(client, market)?;
    println!("--- market ---");
    println!("  long_qty: {}", m.long_qty);
    println!("  short_qty: {}", m.short_qty);
    println!("  matched_qty: {}", m.matched_qty);
    println!("  unmatched_long_lamports: {}", m.unmatched_long_lamports);
    println!("  unmatched_short_lamports: {}", m.unmatched_short_lamports);
    println!("  matched_lamports: {}", m.matched_lamports);
    println!("  entry_price: {}", m.entry_price);
    println!("  settled: {}", m.settled);
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

    let (market, _) = derive_market_pda();
    let expiry_ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs() as i64
        + EXPIRY_OFFSET_SECS;

    initialize_market(&client1, &wallet1, market, expiry_ts)?;

    open_position(
        &client1,
        &wallet1,
        market,
        Side::Long,
        QTY,
        ENTRY_PRICE,
    )?;
    print_market(&client1, market)?;

    open_position(
        &client2,
        &wallet2,
        market,
        Side::Short,
        QTY,
        ENTRY_PRICE,
    )?;
    print_market(&client1, market)?;

    let (long_position, _) = derive_position_pda(&market, &wallet1.pubkey(), Side::Long);
    let (short_position, _) = derive_position_pda(&market, &wallet2.pubkey(), Side::Short);
    let long_pos = fetch_position(&client1, long_position)?;
    let short_pos = fetch_position(&client2, short_position)?;
    println!("long matched_contracts: {}", long_pos.matched_contracts);
    println!("short matched_contracts: {}", short_pos.matched_contracts);

    println!("waiting {EXPIRY_OFFSET_SECS}s for expiry");
    thread::sleep(Duration::from_secs(EXPIRY_OFFSET_SECS as u64));

    set_settlement_price(&client1, &wallet1, market, SETTLEMENT_PRICE)?;

    claim(&client1, &wallet1, market, Side::Long)?;
    claim(&client2, &wallet2, market, Side::Short)?;

    let expected_long = MARGIN_PER_CONTRACT + (SETTLEMENT_PRICE - ENTRY_PRICE);
    let expected_short = MARGIN_PER_CONTRACT.saturating_sub(SETTLEMENT_PRICE - ENTRY_PRICE);
    println!("expected long payout: {expected_long}");
    println!("expected short payout: {expected_short}");

    Ok(())
}
