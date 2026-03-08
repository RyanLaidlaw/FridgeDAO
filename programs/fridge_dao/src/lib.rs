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

    pub fn vote(ctx: Context<Vote>, proposal_id: u64) -> Result<()> {
        instructions::vote(ctx, proposal_id)?;
        Ok(())
    }

    pub fn propose(ctx: Context<Propose>, description: String) -> Result<()> {
        instructions::propose(ctx, description)?;
        Ok(())
    }

    pub fn choose_proposal<'info>(ctx: Context<'_, '_, 'info, 'info, Choose<'info>>) -> Result<()> {
        instructions::choose_proposal(ctx)?;
        Ok(())
    }

    pub fn cancel_proposal(ctx: Context<Cancel>, proposal_id: u64) -> Result<()> {
        instructions::cancel_proposal(ctx, proposal_id)?;
        Ok(())
    }

    pub fn update_vote_cooldown(ctx: Context<UpdateCooldown>, new_cooldown: u64) -> Result<()> {
        instructions::update_vote_cooldown(ctx, new_cooldown)?;
        Ok(())
    }

    pub fn add_valid_addresses(ctx: Context<AddAddress>, new_addresses: Vec<Pubkey>) -> Result<()> {
        instructions::add_valid_addresses(ctx, new_addresses)?;
        Ok(())
    }

    pub fn remove_valid_addresses(ctx: Context<RemoveAddress>, addresses_to_remove: Vec<Pubkey>) -> Result<()> {
        instructions::remove_valid_addresses(ctx, addresses_to_remove)?;
        Ok(())
    }
}
