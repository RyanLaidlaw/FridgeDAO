use anchor_lang::prelude::*;

#[error_code]
pub enum Error {
    #[msg("DAO Mint already initialized")]
    AlreadyInitialized,
    #[msg("Math error")]
    MathOverflowOrUnderflow,
    #[msg("Invalid authority")]
    InvalidAuthority,
    #[msg("Invalid accounts")]
    InvalidAccounts,
    #[msg("Key is already added to valid addresses")]
    KeyAlreadyAdded,
    #[msg("Key does not exist")]
    KeyDoesNotExist,
    #[msg("At maximum members")]
    MaxMembers,
    #[msg("Duplicate key")]
    DuplicateKeys,
    #[msg("Remaining accounts length is not equal to accounts to remove")]
    MismatchingRemainingAccountsLength,
    #[msg("Invalid vault")]
    InvalidTokenAccount,
    #[msg("Invalid token account owner")]
    InvalidTokenAccountOwner,
    #[msg("Invalid mint")]
    InvalidMint,
    #[msg("Invalid vault")]
    InvalidVault,
    #[msg("Invalid member")]
    InvalidMember,
}