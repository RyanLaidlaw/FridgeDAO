use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;
use std::collections::HashSet;

use crate::{error, state};

// DEV: Pass users token accounts into remaining_accounts
#[derive(Accounts)]
pub struct WhitelistKeys<'info> {
    #[account(mut)]
    pub adder: Signer<'info>,

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
}

pub fn whitelist_keys(ctx: Context<WhitelistKeys>, new_keys: Vec<Pubkey>) -> Result<()> {
    let dao_mint = &mut ctx.accounts.dao_mint;
    let remaining = ctx.remaining_accounts;

    require!(remaining.len() == new_keys.len(), error::Error::MismatchingRemainingAccountsLength);
    require!(dao_mint.valid_member_keys.len() + new_keys.len() <= state::MAX_MEMBERS, error::Error::MaxMembers);
    require!(ctx.accounts.adder.key() == dao_mint.admin, error::Error::InvalidAuthority);

    let existing: HashSet<Pubkey> = dao_mint.valid_member_keys.iter().map(|u: &state::UserWithTokenAcct| u.key).collect();
    let mut seen: HashSet<Pubkey> = HashSet::new();

    for k in &new_keys {
        require!(!existing.contains(k), error::Error::KeyAlreadyAdded);
        require!(seen.insert(*k), error::Error::DuplicateKeys);
    }

    let mut token_account_amounts: Vec<u64> = Vec::new();
    
    let token_accounts: Vec<Pubkey> = remaining
        .iter()
        .zip(new_keys.iter())
        .map(|(acct, key)| {
            let token_acct = TokenAccount::try_deserialize(&mut acct.try_borrow_data()?.as_ref())?;
            require!(token_acct.owner == *key, error::Error::InvalidTokenAccount);
            require!(token_acct.mint == dao_mint.usdc_mint, error::Error::InvalidMint);
            token_account_amounts.push(token_acct.amount);
            Ok(acct.key())
        })
        .collect::<Result<Vec<Pubkey>>>()?;

    dao_mint.valid_member_keys.extend(
        new_keys.iter().zip(token_accounts.iter()).map(|(addr, token_acct)| {
            state::UserWithTokenAcct {
                key: *addr,
                token_account: *token_acct,
            }
        })
    );

    let seeds = &[
        state::DAOMint::SEED_PREFIX,
        &[ctx.bumps.dao_mint][..],
    ];
    let signer_seeds = &[&seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.dao_program.to_account_info(),
        fridge_dao::cpi::accounts::DAOWhitelist {
            signer: ctx.accounts.dao_mint.to_account_info(),
            fridge_dao: ctx.accounts.fridge_dao.to_account_info()
        },
        signer_seeds
    );

    fridge_dao::cpi::whitelist(cpi_ctx, new_keys, token_account_amounts)?;

    Ok(())
}