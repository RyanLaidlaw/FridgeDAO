use anchor_lang::prelude::*;
use anchor_spl::token_interface::{TransferChecked, transfer_checked};

use crate::state;

pub fn transfer<'info>(
    to: AccountInfo<'info>,
    from: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    mint: AccountInfo<'info>,
    program: AccountInfo<'info>,
    amount: u64)
    -> Result<()> {
    let cpi_accounts = TransferChecked {
        from: from,
        to: to,
        authority: authority,
        mint: mint,
    };

    let cpi_ctx = CpiContext::new(
        program,
        cpi_accounts,
    );

    transfer_checked(cpi_ctx, amount, state::DECIMALS)?;

    Ok(())
}