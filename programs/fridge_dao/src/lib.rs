use anchor_lang::prelude::*;

declare_id!("3Mtgqr9jRqBMAbp9kTde56EWvGhJk744vSqiv9eJmX7q");

#[program]
pub mod fridge_dao {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
