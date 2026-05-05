use anchor_lang::prelude::*;

declare_id!("4iLCBjB3g5uEzAjepzCBfNACuxrLfxKM1JvT8GV5xzmd");

pub mod state;
pub mod error;
pub mod instructions;

#[program]
pub mod dao_mint {
    pub use super::instructions::*;
    use super::*;

    pub fn initialize(ctx: Context<InitMint>, vote_period_len: u64, time_until_first_vote: u64) -> Result<()> {
        instructions::init_mint(ctx, vote_period_len, time_until_first_vote)?;
        Ok(())
    }

    pub fn whitelist(ctx: Context<WhitelistKeys>, new_keys: Vec<Pubkey>) -> Result<()> {
        instructions::whitelist_keys(ctx, new_keys)?;
        Ok(())
    }

    pub fn blacklist<'info>(ctx: Context<'_, '_, '_, 'info , BlacklistKeys<'info>>, keys_to_blacklist: Vec<Pubkey>) -> Result<()> {
        instructions::blacklist_keys(ctx, keys_to_blacklist)?;
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

    pub fn withdraw_proposal_amount<'info>(ctx: Context<'_, '_, 'info, 'info, WithdrawProposalAmt<'info>>) -> Result<()> {
        instructions::withdraw_proposal_amount(ctx)?;
        Ok(())
    }
}
