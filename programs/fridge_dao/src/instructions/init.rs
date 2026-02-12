use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;

use crate::error;
use crate::state;

#[derive(Accounts)]
pub struct InitDAO<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + state::FridgeDao::INIT_SPACE,
        seeds = [
            state::FridgeDao::SEED_PREFIX,
            authority.key().as_ref(),
        ],
        bump,
    )]
    pub fridge_dao: Account<'info, state::FridgeDao>,

    #[account(
        seeds = [b"treasury", fridge_dao.key().as_ref()],
        bump
    )]
    pub treasury: SystemAccount<'info>,

    pub usdc_mint: InterfaceAccount<'info, Mint>,

    pub system_program: Program<'info, System>,
}

pub fn init_dao(ctx: Context<InitDAO>) -> Result<()> {
    let dao = &mut ctx.accounts.fridge_dao;

    dao.authority = ctx.accounts.authority.key();
    dao.treasury = ctx.accounts.treasury.key();
    dao.usdc_mint = ctx.accounts.usdc_mint.key();
    dao.proposal_count = 0;

    Ok(())
}