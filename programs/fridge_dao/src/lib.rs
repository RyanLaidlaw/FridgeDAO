use anchor_lang::prelude::*;

pub mod error;
pub mod state;
pub mod instructions;

declare_id!("3Mtgqr9jRqBMAbp9kTde56EWvGhJk744vSqiv9eJmX7q");

#[program]
pub mod fridge_dao {
    pub use super::instructions::*;
    use super::*;

    pub fn initialize(ctx: Context<InitDAO>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}
