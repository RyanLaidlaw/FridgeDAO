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

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        instructions::deposit(ctx, amount)?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        instructions::withdraw(ctx, amount)?;
        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, proposal_id: u64) -> Result<()> {
        instructions::vote(ctx, proposal_id)?;
        Ok(())
    }

    pub fn propose(ctx: Context<Propose>, description: String, price: u64) -> Result<()> {
        instructions::propose(ctx, description, price)?;
        Ok(())
    }

    pub fn choose_proposal<'info>(ctx: Context<'_, '_, 'info, 'info, Choose<'info>>) -> Result<()> {
        instructions::choose_proposal(ctx)?;
        Ok(())
    }

    pub fn cancel_proposal(ctx: Context<Cancel>, _proposal_id: u64) -> Result<()> {
        instructions::cancel_proposal(ctx)?;
        Ok(())
    }

    pub fn update_vote_cooldown(ctx: Context<UpdateCooldown>, new_cooldown: u64) -> Result<()> {
        instructions::update_vote_cooldown(ctx, new_cooldown)?;
        Ok(())
    }

    pub fn whitelist(ctx: Context<DAOWhitelist>, new_keys: Vec<Pubkey>, token_account_amounts: Vec<u64>) -> Result<()> {
        instructions::whitelist_keys(ctx, new_keys, token_account_amounts)?;
        Ok(())
    }

    pub fn blacklist<'info>(ctx: Context<DAOBlacklist>, keys_to_remove: Vec<Pubkey>) -> Result<()> {
        instructions::blacklist_keys(ctx, keys_to_remove)?;
        Ok(())
    }

    pub fn resolve_tie(ctx: Context<ResolveTie>) -> Result<()> {
        instructions::resolve_tie(ctx)?;
        Ok(())
    }

    pub fn retrieve_winner(ctx: Context<RetrieveWinner>) -> Result<()> {
        instructions::retrieve_winner(ctx)?;
        Ok(())
    }
}
