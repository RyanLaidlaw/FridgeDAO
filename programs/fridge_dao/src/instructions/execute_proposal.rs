use anchor_lang::prelude::*;

use crate::error;

#[derive(Accounts)]
pub struct Execute { // <'info>

}

pub fn execute_proposal(ctx: Context<Execute>) -> Result<()> {
    Ok(())
}