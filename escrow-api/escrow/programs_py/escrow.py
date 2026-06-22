# escrow
# Timed SOL escrow: deposit, then release back to depositor after 5 minutes.

from seahorse.prelude import *

declare_id('JBSsJbhEHwBBy3NC9fxw33Up4PaTPHXHKoVYME7RxeSb')

LOCK_SECONDS = 300


class Escrow(Account):
    owner: Pubkey
    amount: u64
    unlock_at: i64
    bump: u8


@instruction
def deposit(depositor: Signer, escrow: Empty[Escrow], amount: u64, clock: Clock):
    assert amount > 0, 'amount must be positive'

    bump = escrow.bump()
    vault = escrow.init(payer=depositor, seeds=['escrow', depositor])

    vault.owner = depositor.key()
    vault.amount = amount
    vault.unlock_at = clock.unix_timestamp() + LOCK_SECONDS
    vault.bump = bump

    depositor.transfer_lamports(vault, amount)


@instruction
def release(depositor: Signer, escrow: Escrow, clock: Clock):
    assert depositor.key() == escrow.owner, 'not the depositor'
    assert clock.unix_timestamp() >= escrow.unlock_at, 'escrow still locked'
    assert escrow.amount > 0, 'nothing to release'

    amount = escrow.amount
    escrow.amount = 0
    escrow.transfer_lamports(depositor, amount)
