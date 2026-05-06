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

    #[account(
        mut,
        close = proposer,
        seeds = [
            state::Proposal::SEED_PREFIX,
            fridge_dao.key().as_ref(),
            winning_proposal.identifier.to_le_bytes().as_ref(),
        ],
        bump = winning_proposal.bump,
    )]
    pub winning_proposal: Account<'info, state::Proposal>,

    /// CHECK: validated against winner_proposal.proposer
    #[account(
        mut,
        constraint = proposer.key() == winning_proposal.proposer @ error::Error::InvalidProposal
    )]
    pub proposer: UncheckedAccount<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn retrieve_winner(ctx: Context<RetrieveWinner>) -> Result<()> {
    let dao = &mut ctx.accounts.fridge_dao;

    require!(ctx.accounts.retriever.key() == dao.mint_admin_program, error::Error::InvalidAuthority);

    let now = Clock::get()?.unix_timestamp;
    let voting_end = lib::get_voting_end(&dao)?;

    require!(now > voting_end, error::Error::VotingHasNotEnded);

    let winner: state::Proposal = dao.recent_winner.clone().ok_or(error::Error::NoWinner)?;

    require!(ctx.accounts.winning_proposal.identifier == winner.identifier, error::Error::InvalidProposal);

    let output = &mut ctx.accounts.winner_output;
    output.identifier = winner.identifier;
    output.voters = winner.voters;
    output.proposer = winner.proposer;
    output.description = winner.description;
    output.price = winner.price;
    output.bump = winner.bump;

    dao.proposal_count -= 1;
    dao.proposals.retain(|x|  *x != ctx.accounts.winning_proposal.key());
    dao.recent_winner = None;

    Ok(())
}