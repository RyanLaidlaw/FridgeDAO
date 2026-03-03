use anchor_lang::prelude::*;

use crate::error;

#[derive(Accounts)]
pub struct Deposit {

}

pub fn deposit(ctx: Context<Deposit>) -> Result<()> {
    //invalid member
    Ok(())
}