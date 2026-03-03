use anchor_lang::prelude::*;

use crate::error;

#[derive(Accounts)]
pub struct RemoveAddress {

}

pub fn remove_valid_address(ctx: Context<RemoveAddress>) -> Result<()> {
    //only authority
    Ok(())
}