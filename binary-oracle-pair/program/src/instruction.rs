//! Instruction types

//use crate::{
//    error::PoolError,
//};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use solana_program::{
    //instruction::{AccountMeta},
    //program_error::ProgramError,
    pubkey::Pubkey,
    //sysvar,
};
//use std::{convert::TryInto, mem::size_of};

#[derive(Clone, Debug, PartialEq)]
pub enum Instruction {

    /// Initializes a new binary oracle pair pool.
    ///
    ///   0. `[writable]` Pool account.
    ///   1. `[]` authority create_program_address(&[Token-swap account])`
    ///   1. `[]` Deposit currency SPL Token mint. Must be initialized.
    ///   2. `[]` Rent sysvar
    ///   3. '[]` Token program id
    InitPool {
        /// authority that decides the result
        decider: Pubkey,
        /// mint end slot
        mint_end_slot: Slot,
        /// decide end slot
        decide_end_slot: Slot,
        /// authority nonce
        nonce: u8,
    },

    ///   Deposit in the pool.
    ///
    ///   0. `[]` Pool
    ///   1. `[]` authority
    ///   2. `[]` user transfer authority
    ///   3. `[writable]` token SOURCE Account, amount is transferable by user transfer authority,
    ///   4. `[writable]` token_P PASS mint
    ///   5. `[writable]` token_F FAIL mint
    ///   6. `[writable]` token_P DESTINATION Account assigned to USER as the owner.
    ///   7. `[writable]` token_F DESTINATION Account assigned to USER as the owner.
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
    ///   2. `[]` user transfer authority
    ///   4. `[writable]` token_P PASS SOURCE Account
    ///   5. `[writable]` token_F FAIL SOURCE Account
    ///   4. `[writable]` token_P PASS DESTINATION mint
    ///   5. `[writable]` token_F FAIL DESTINATION mint
    ///   7. `[writable]` deposit SOURCE Account 
    ///   7. `[writable]` deposit DESTINATION Account assigned to USER as the owner.
    ///   8. '[]` Token program id
    ///   9. '[]` Sysvar Clock
    Withdraw(u64),

    ///  Trigger the decision.
    ///  Call only succeeds once and if current slot > mint_end slot AND < decide_end slot 
    ///   0. `[]` Pool
    ///   1. `[signer]` decider pubkey
    ///   2. '[]` Sysvar Clock
    Decide(bool),
}
