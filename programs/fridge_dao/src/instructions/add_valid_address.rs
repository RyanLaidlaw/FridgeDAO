use anchor_lang::prelude::*;

use crate::error;

#[derive(Accounts)]
pub struct AddAddress {

}

pub fn add_valid_address(ctx: Context<AddAddress>) -> Result<()> {
    //only authority
    Ok(())
}