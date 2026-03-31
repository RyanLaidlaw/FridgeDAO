use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
use anchor_spl::associated_token::get_associated_token_address;

use crate::{error, state};

#[derive(Accounts)]
pub struct InitMint<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = 8 + state::DAOMint::INIT_SPACE,
        seeds = [state::DAOMint::SEED_PREFIX,],
        bump,
    )]
    pub dao_mint: Account<'info, state::DAOMint>,

    #[account(
        init,
        payer = admin,
        token::mint = usdc_mint,
        token::authority = dao_mint,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub usdc_mint: InterfaceAccount<'info, Mint>,

    pub dao_program: Program<'info, fridge_dao::program::FridgeDao>,

    /// CHECK: This account is verified by the CPI to the fridge_dao program, where the seeds and bump are checked
    #[account(mut)]
    pub fridge_dao: UncheckedAccount<'info>,

    #[account(
        mut,
        constraint = admin_token_account.owner == admin.key() @ error::Error::InvalidTokenAccount
    )]
    pub admin_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn init_mint(ctx: Context<InitMint>, vote_period_len: u64, time_until_first_vote: u64) -> Result<()> {
    let expected_ata = get_associated_token_address(&ctx.accounts.admin.key(), &ctx.accounts.usdc_mint.key());
    require!(ctx.accounts.admin_token_account.key() == expected_ata, error::Error::InvalidTokenAccount);

    {
        let dao_mint = &mut ctx.accounts.dao_mint;
        let admin = &ctx.accounts.admin;

        dao_mint.admin = admin.key();
        dao_mint.vault = ctx.accounts.vault.key();
        dao_mint.usdc_mint = ctx.accounts.usdc_mint.key();
        dao_mint.dao_program = ctx.accounts.dao_program.key();
        dao_mint.bump = ctx.bumps.dao_mint;
        dao_mint.valid_member_keys.push(
            state::UserWithTokenAcct{
                key: admin.key(),
                token_account: ctx.accounts.admin_token_account.key()
            }
        );
    }

    let seeds = &[
        state::DAOMint::SEED_PREFIX,
        &[ctx.bumps.dao_mint][..],
    ];
    let signer_seeds = &[&seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.dao_program.to_account_info(),
        fridge_dao::cpi::accounts::InitDAO {
            dao_mint: ctx.accounts.dao_mint.to_account_info(),
            admin: ctx.accounts.admin.to_account_info(),
            fridge_dao: ctx.accounts.fridge_dao.to_account_info(),
            admin_token_account: ctx.accounts.admin_token_account.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        },
        signer_seeds,
    );

    fridge_dao::cpi::initialize(cpi_ctx, vote_period_len, time_until_first_vote)?;

    Ok(())
}