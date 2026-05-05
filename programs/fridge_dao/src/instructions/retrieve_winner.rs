use anchor_lang::prelude::*;

use crate::{error, state, instructions::lib};

#[derive(Accounts)]
pub struct RetrieveWinner<'info> {
    #[account(mut)]
    pub retriever: Signer<'info>,

    #[account(
        mut,
        seeds = [state::FridgeDao::SEED_PREFIX,],
        bump,
    )]
    pub fridge_dao: Account<'info, state::FridgeDao>,
    
    #[account(
        init_if_needed,
        payer = retriever,
        space = 8 + state::Proposal::INIT_SPACE,
        seeds = [b"winner_output"],
        bump,
    )]
    pub winner_output: Account<'info, state::Proposal>, 
    
    pub system_program: Program<'info, System>,
}

pub fn retrieve_winner(ctx: Context<RetrieveWinner>) -> Result<()> {
    let dao = &mut ctx.accounts.fridge_dao;

    require!(ctx.accounts.retriever.key() == dao.mint_admin_program, error::Error::InvalidAuthority);

    let now = Clock::get()?.unix_timestamp;
    let voting_end = lib::get_voting_end(&dao)?;

    require!(now > voting_end, error::Error::VotingHasNotEnded);

    let winner: state::Proposal = dao.recent_winner.clone().ok_or(error::Error::NoWinner)?;

    let output = &mut ctx.accounts.winner_output;
    output.identifier = winner.identifier;
    output.voters = winner.voters;
    output.proposer = winner.proposer;
    output.description = winner.description;
    output.bump = winner.bump;
    output.price = winner.price;

    Ok(())
}