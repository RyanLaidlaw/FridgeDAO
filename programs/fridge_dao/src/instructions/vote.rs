use anchor_lang::prelude::*;

use crate::error;

#[derive(Accounts)]
pub struct Vote {

}

pub fn vote(ctx: Context<Vote>) -> Result<()> {
    Ok(())
}