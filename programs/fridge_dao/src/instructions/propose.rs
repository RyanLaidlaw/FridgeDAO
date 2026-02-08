use anchor_lang::prelude::*;

use crate::error;

#[derive(Accounts)]
pub struct Propose {

}

pub fn propose(ctx: Context<Propose>) -> Result<()> {
    Ok(())
}