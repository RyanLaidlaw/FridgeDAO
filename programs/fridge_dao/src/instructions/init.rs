use anchor_lang::prelude::*;

use crate::error;

#[derive(Accounts)]
pub struct InitDAO { // <'info>

}

pub fn init_dao(ctx: Context<InitDAO>) -> Result<()> {
    Ok(())
}