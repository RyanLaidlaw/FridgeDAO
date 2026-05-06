use anchor_lang::prelude::*;
use anchor_spl::token_interface::{TokenAccount, TokenInterface, Mint};

use crate::{error, state, instructions::lib};

// DEV: Pass in all members as remaining accounts
#[derive(Accounts)]
pub struct WithdrawProposalAmt<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [state::DAOMint::SEED_PREFIX,],
        bump,
    )]
    pub dao_mint: Account<'info, state::DAOMint>,

    pub dao_program: Program<'info, fridge_dao::program::FridgeDao>,

    /// CHECK: This account is verified by the CPI to the fridge_dao program, where the seeds and bump are checked
    #[account(mut)]
    pub fridge_dao: UncheckedAccount<'info>,

    #[account(
        mut,
        constraint = vault.key() == dao_mint.vault @ error::Error::InvalidVault
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        constraint = admin_token_account.owner == admin.key() @ error::Error::InvalidTokenAccount
    )]
    pub admin_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"winner_output"],
        bump,
        seeds::program = fridge_dao::ID,
    )]
    pub winner_output: Account<'info, fridge_dao::state::Proposal>,

    /// CHECK: validated by the CPI against dao.recent_winner
    #[account(mut)]
    pub winning_proposal: UncheckedAccount<'info>,

    /// CHECK: validated by the CPI against winning_proposal.proposer
    #[account(mut)]
    pub proposer: UncheckedAccount<'info>,

    pub usdc_mint: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn withdraw_proposal_amount<'info>(ctx: Context<'_, '_, 'info, 'info, WithdrawProposalAmt<'info>>) -> Result<()> {
    require!(ctx.accounts.admin.key() == ctx.accounts.dao_mint.admin, error::Error::InvalidAuthority);

    let seeds = &[
        state::DAOMint::SEED_PREFIX,
        &[ctx.bumps.dao_mint][..],
    ];

    let signer_seeds = &[&seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.dao_program.to_account_info(),
        fridge_dao::cpi::accounts::RetrieveWinner {
            retriever: ctx.accounts.dao_mint.to_account_info(),
            fridge_dao: ctx.accounts.fridge_dao.to_account_info(),
            winner_output: ctx.accounts.winner_output.to_account_info(),
            winning_proposal: ctx.accounts.winning_proposal.to_account_info(),
            proposer: ctx.accounts.proposer.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        },
        signer_seeds
    );

    fridge_dao::cpi::retrieve_winner(cpi_ctx)?;

    ctx.accounts.winner_output.reload()?;
    let price = ctx.accounts.winner_output.price;

    require!(ctx.accounts.vault.amount >= price, error::Error::InsufficientFunds);

    lib::transfer(
        ctx.accounts.vault.to_account_info(),
        ctx.accounts.admin_token_account.to_account_info(),
        ctx.accounts.dao_mint.to_account_info(),
        ctx.accounts.usdc_mint.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        price,
        signer_seeds
    )?;

    Ok(())
}
