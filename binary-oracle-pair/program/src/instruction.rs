//! Instruction types

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::clock::Slot;

/// Initialize arguments for pool
#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct InitArgs {
    /// mint end slot
    pub mint_end_slot: Slot,
    /// decide end slot
    pub decide_end_slot: Slot,
    /// authority nonce
    pub bump_seed: u8,
}

/// Instruction definition
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub enum Instruction {
    /// Initializes a new binary oracle pair pool.
    ///
    ///   0. `[w]` Pool account.
    ///   1. `[]` authority create_program_address(&[binary-oracle-pair account])`
    ///   2. `[]` Decider authority
    ///   3. `[]` Deposit currency SPL Token mint. Must be initialized.
    ///   4. `[]` Rent sysvar
    ///   5. '[]` Token program id
    InitPool(InitArgs),

    ///   Deposit in the pool.
    ///
    ///   0. `[]` Pool
    ///   1. `[]` authority
    ///   2. `[]` user transfer authority - don't need
    ///   3. `[w]` token SOURCE Account, amount is transferable by user transfer authority,
    ///   4. `[w]` token_P PASS mint
    ///   5. `[w]` token_F FAIL mint
    ///   6. `[w]` token_P DESTINATION Account assigned to USER as the owner.
    ///   7. `[w]` token_F DESTINATION Account assigned to USER as the owner.
    ///   8. '[]` Token program id
    Deposit(u64),

    ///   Withdraw from the pool.
    ///   If current slot is < mint_end slot, 1 Pass AND 1 Fail token convert to 1 deposit
    ///   If current slot is > mint_end slot && decide == Some(true), 1 Pass convert to 1 deposit
    ///   otherwise 1 Fail converts to 1 deposit
    ///
    ///   Pass tokens convert 1:1 to the deposit token iff decision is set to Some(true)
    ///   AND current slot is > decide_end_slot.
    ///
    ///   0. `[]` Pool
    ///   1. `[]` authority
    ///   2. `[]` user transfer authority - don't need
    ///   4. `[w]` token_P PASS SOURCE Account
    ///   5. `[w]` token_F FAIL SOURCE Account
    ///   4. `[w]` token_P PASS DESTINATION mint
    ///   5. `[w]` token_F FAIL DESTINATION mint
    ///   7. `[w]` deposit SOURCE Account
    ///   7. `[w]` deposit DESTINATION Account assigned to USER as the owner.
    ///   8. '[]` Token program id
    ///   9. '[]` Sysvar Clock
    Withdraw(u64),

    ///  Trigger the decision.
    ///  Call only succeeds once and if current slot > mint_end slot AND < decide_end slot
    ///   0. `[]` Pool
    ///   1. `[s]` decider pubkey
    ///   2. '[]` Sysvar Clock
    Decide(bool),
}
