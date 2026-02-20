use anchor_lang::prelude::*;

pub mod error;
pub mod state;
pub mod instructions;

declare_id!("3Mtgqr9jRqBMAbp9kTde56EWvGhJk744vSqiv9eJmX7q");

#[program]
pub mod fridge_dao {
    pub use super::instructions::*;
    use super::*;

    pub fn initialize(ctx: Context<InitDAO>) -> Result<()> {
        instructions::initialize(ctx)?;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>) -> Result<()> {
        instructions::deposit(ctx)?;
        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, proposal_number: u64, vote_choice: state::VoteChoice) -> Result<()> {
        instructions::vote(ctx, proposal_number, vote_choice)?;
        Ok(())
    }

    pub fn propose(ctx: Context<Propose>, description: String) -> Result<()> {
        instructions::propose(ctx, description)?;
        Ok(())
    }

    pub fn execute_proposal(ctx: Context<Execute>) -> Result<()> {
        instructions::execute_proposal(ctx)?;
        Ok(())
    }

    pub fn cancel(ctx: Context<Cancel>) -> Result<()> {
        instructions::cancel(ctx)?;
        Ok(())
    }
}
