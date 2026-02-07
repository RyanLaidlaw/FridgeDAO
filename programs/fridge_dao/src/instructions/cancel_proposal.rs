use anchor_lang::prelude::*;

use crate::error;

#[derive(Accounts)]
pub struct Cancel {

}

pub fn cancel(ctx: Context<Cancel>) -> Result<()> {
    Ok(())
}