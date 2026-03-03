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
}