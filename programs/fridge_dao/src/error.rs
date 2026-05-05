use anchor_lang::prelude::*;

#[error_code]
pub enum Error {
    #[msg("Math error")]
    MathOverflowOrUnderflow,
    #[msg("Invalid authority")]
    InvalidAuthority,
    #[msg("Invalid member")]
    InvalidMember,
    #[msg("Invalid proposal")]
    InvalidProposal,
    #[msg("DAO is not in a voting period")]
    NotInVotingPeriod,
    #[msg("DAO voting period has not ended")]
    VotingHasNotEnded,
    #[msg("Could not find proposal")]
    CouldNotFindProposal,
    #[msg("User already voted")]
    AlreadyVoted,
    #[msg("New cooldown must be different")]
    IdenticalCooldown,
    #[msg("Too many proposals")]
    AtMaxProposals,
    #[msg("Key is already added to valid addresses")]
    KeyAlreadyAdded,
    #[msg("Key does not exist")]
    KeyDoesNotExist,
    #[msg("At maximum members")]
    MaxMembers,
    #[msg("Duplicate key")]
    DuplicateKeys,
    #[msg("Invalid vault")]
    InvalidVault,
    #[msg("Invalid mint")]
    InvalidMint,
    #[msg("Remaining accounts length is not equal to accounts to remove")]
    MismatchingRemainingAccountsLength,
    #[msg("Invalid token account owner")]
    InvalidTokenAccountOwner,
    #[msg("Token account does not match ")]
    InvalidTokenAccount,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("There is no tie to be resolved")]
    NoTie,
    #[msg("This proposal is not one of the tied proposals")]
    ProposalNotInTies,
    #[msg("Winner has not been chosen yet")]
    NoWinner,
}
