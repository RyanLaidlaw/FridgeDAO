use anchor_lang::prelude::*;

#[error_code]
pub enum Error {
    #[msg("Invalid proposal")]
    InvalidProposal,
    #[msg("Voting period has ended")]
    VotingPeriodEnded,
    #[msg("Voting period has not stared")]
    VotingPeriodNotStarted,
    #[msg("Could not find proposal")]
    CouldNotFindProposal,
}