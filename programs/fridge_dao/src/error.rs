use anchor_lang::prelude::*;

#[error_code]
pub enum Error {
    #[msg("Math error")]
    MathOverflowOrUnderflow,
    #[msg("Invalid authority")]
    InvalidAuthority,
    #[msg("Invalid proposal")]
    InvalidProposal,
    #[msg("DAO is not in a voting period")]
    NotInVotingPeriod,
    #[msg("Could not find proposal")]
    CouldNotFindProposal,
    #[msg("User already voted")]
    AlreadyVoted,
    #[msg("New cooldown must be different")]
    IdenticalCooldown,
}