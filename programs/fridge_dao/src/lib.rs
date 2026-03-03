use anchor_lang::prelude::*;

pub mod error;
pub mod state;
pub mod instructions;

declare_id!("3Mtgqr9jRqBMAbp9kTde56EWvGhJk744vSqiv9eJmX7q");

#[program]
pub mod fridge_dao {
    pub use super::instructions::*;
    use super::*;

    pub fn initialize(ctx: Context<InitDAO>, vote_period_len: u64, time_until_first_vote: u64) -> Result<()> {
        instructions::initialize(ctx, vote_period_len, time_until_first_vote)?;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>) -> Result<()> {
        instructions::deposit(ctx)?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        instructions::withdraw(ctx)?;
        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, proposal_number: u64) -> Result<()> {
        instructions::vote(ctx, proposal_number)?;
        Ok(())
    }

    pub fn propose(ctx: Context<Propose>, description: String) -> Result<()> {
        instructions::propose(ctx, description)?;
        Ok(())
    }

    pub fn choose_proposal(ctx: Context<Choose>) -> Result<()> {
        instructions::choose_proposal(ctx)?;
        Ok(())
    }

    pub fn cancel(ctx: Context<Cancel>) -> Result<()> {
        instructions::cancel(ctx)?;
        Ok(())
    }

    pub fn update_vote_cooldown(ctx: Context<UpdateCooldown>, new_cooldown: u64) -> Result<()> {
        instructions::update_vote_cooldown(ctx, new_cooldown)?;
        Ok(())
    }

    pub fn add_vald_address(ctx: Context<AddAddress>) -> Result<()> {
        instructions::add_valid_address(ctx)?;
        Ok(())
    }

    pub fn remove_vald_address(ctx: Context<RemoveAddress>) -> Result<()> {
        instructions::remove_valid_address(ctx)?;
        Ok(())
    }
}
