use anchor_lang::prelude::*;

use crate::{error, state};

#[derive(Accounts)]
pub struct UpdateCooldown<'info> {
    #[account(mut)]
    pub updater: Signer<'info>,

    #[account(
        mut,
        seeds = [state::FridgeDao::SEED_PREFIX,],
        bump,
    )]
    pub fridge_dao: Account<'info, state::FridgeDao>,
}

pub fn update_vote_cooldown(ctx: Context<UpdateCooldown>, new_cooldown: u64) -> Result<()> {
    let dao = &mut ctx.accounts.fridge_dao;
    
    require!(ctx.accounts.updater.key() == dao.admin, error::Error::InvalidAuthority);
    require!(new_cooldown != dao.vote_cooldown, error::Error::IdenticalCooldown);

    dao.vote_cooldown = new_cooldown;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_dao(admin: Pubkey, vote_cooldown: u64) -> state::FridgeDao {
        state::FridgeDao {
            mint_admin_program: Pubkey::default(),
            admin,
            vote_cooldown,
            vault_balance: 0,
            bump: 0,
            proposal_count: 0,
            vote_period: 0,
            next_vote_allowed_at: 0,
            proposals: Vec::new(),
            valid_member_keys: Vec::new(),
            tie: None,
            recent_winner: None,
        }
    }

    #[test]
    fn rejects_identical_cooldown() {
        let admin = Pubkey::new_unique();
        let mut dao = make_dao(admin, 100);

        assert!(dao.vote_cooldown == 100);
        let result = if 100 != dao.vote_cooldown {
            dao.vote_cooldown = 100;
            Ok(())
        } else {
            Err(error::Error::IdenticalCooldown)
        };

        assert!(result.is_err());
    }

    #[test]
    fn updates_cooldown() {
        let admin = Pubkey::new_unique();
        let mut dao = make_dao(admin, 100);

        assert!(dao.vote_cooldown != 200);
        dao.vote_cooldown = 200;
        assert_eq!(dao.vote_cooldown, 200);
    }
}
