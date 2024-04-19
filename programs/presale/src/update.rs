use anchor_lang::prelude::*;

use crate::errors::PresaleError;
use crate::account::*;

pub fn update(
    ctx: Context<UpdateStartEnd>,
    start_time: u64,
    end_time: u64,
) -> Result<()> {    
    let presale_account = &mut ctx.accounts.presale_account;

    require!(ctx.accounts.payer.key() == presale_account.owner.key(), PresaleError::IllegalOwner);

    presale_account.start_time = start_time;
    presale_account.end_time = end_time;

    Ok(())
}

#[derive(Accounts)]
pub struct UpdateStartEnd<'info> {
    #[account(mut)]
    pub presale_account: Account<'info, PresaleAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
}