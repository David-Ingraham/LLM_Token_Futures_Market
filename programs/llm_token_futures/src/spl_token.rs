use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::invoke,
    program::invoke_signed,
    pubkey::Pubkey,
};

/// SPL Token program (mainnet/devnet).
pub const ID: Pubkey = solana_program::pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

pub const ACCOUNT_LEN: usize = 165;

/// Transfer: instruction index 3.
pub fn transfer<'a>(
    token_program: &AccountInfo<'a>,
    source: &AccountInfo<'a>,
    destination: &AccountInfo<'a>,
    authority: &AccountInfo<'a>,
    signer_seeds: &[&[&[u8]]],
    amount: u64,
) -> ProgramResult {
    let mut data = vec![3u8];
    data.extend_from_slice(&amount.to_le_bytes());
    let ix = Instruction {
        program_id: ID,
        accounts: vec![
            AccountMeta::new(*source.key, false),
            AccountMeta::new(*destination.key, false),
            AccountMeta::new_readonly(*authority.key, true),
        ],
        data,
    };
    if signer_seeds.is_empty() {
        invoke(
            &ix,
            &[source.clone(), destination.clone(), authority.clone(), token_program.clone()],
        )
    } else {
        invoke_signed(
            &ix,
            &[source.clone(), destination.clone(), authority.clone(), token_program.clone()],
            signer_seeds,
        )
    }
}

/// InitializeAccount: instruction index 1.
pub fn initialize_account<'a>(
    token_program: &AccountInfo<'a>,
    account: &AccountInfo<'a>,
    mint: &AccountInfo<'a>,
    owner: &AccountInfo<'a>,
    rent: &AccountInfo<'a>,
    signer_seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = Instruction {
        program_id: ID,
        accounts: vec![
            AccountMeta::new(*account.key, false),
            AccountMeta::new_readonly(*mint.key, false),
            AccountMeta::new_readonly(*owner.key, false),
            AccountMeta::new_readonly(*rent.key, false),
        ],
        data: vec![1],
    };
    invoke_signed(
        &ix,
        &[
            account.clone(),
            mint.clone(),
            owner.clone(),
            rent.clone(),
            token_program.clone(),
        ],
        signer_seeds,
    )
}
