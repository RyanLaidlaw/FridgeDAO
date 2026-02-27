use anchor_lang::prelude::*;

use crate::error;

#[derive(Accounts)]
pub struct Withdraw {

}

pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
    Ok(())
}