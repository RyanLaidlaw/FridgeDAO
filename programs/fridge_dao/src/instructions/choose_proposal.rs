use anchor_lang::prelude::*;

use crate::error;

#[derive(Accounts)]
pub struct Choose { // <'info>

}

pub fn choose_proposal(ctx: Context<Choose>) -> Result<()> {
    Ok(())
}