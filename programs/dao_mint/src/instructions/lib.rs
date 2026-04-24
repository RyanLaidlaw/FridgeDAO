use anchor_lang::prelude::*;
use anchor_spl::token_interface::{TransferChecked, transfer_checked};

use crate::state;

pub fn transfer<'info>(
    from: AccountInfo<'info>,
    to: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    mint: AccountInfo<'info>,
    program: AccountInfo<'info>,
    amount: u64,
    signer_seeds: &[&[&[u8]]])
    -> Result<()> {
    let cpi_accounts = TransferChecked {
        from: from,
        to: to,
        authority: authority,
        mint: mint,
    };

    let cpi_ctx = CpiContext::new_with_signer(
        program,
        cpi_accounts,
        signer_seeds,
    );

    transfer_checked(cpi_ctx, amount, state::DECIMALS)?;

    Ok(())
}