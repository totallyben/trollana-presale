use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;
use anchor_lang::solana_program::program::invoke;

declare_id!("DcX7NVTtmtUtfmiNQ3bZwov9bKDRpjj7Z8bY61MYPdew");

#[program]
pub mod presale {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, start_time: u64, end_time: u64) -> Result<()> {
        let presale_account = &mut ctx.accounts.presale_account;
        presale_account.start_time = start_time;
        presale_account.end_time = end_time;
        presale_account.is_active = true;
        Ok(())
    }

    pub fn buy_tokens(ctx: Context<BuyTokens>, amount: u64) -> Result<()> {
        let presale_account = &mut ctx.accounts.presale_account;
        let clock = Clock::get().unwrap();

        // Check if the presale is active and within the time bounds
        require!(presale_account.is_active, PresaleError::PresaleNotActive);
        require!(clock.unix_timestamp as u64 >= presale_account.start_time, PresaleError::PresaleNotStarted);
        require!(clock.unix_timestamp as u64 <= presale_account.end_time, PresaleError::PresaleEnded);

        // Calculate the amount of SOL to be transferred based on a predefined rate (e.g., 1 Token = 1 SOL for simplicity)
        let sol_amount = amount; // Assuming 1:1 rate for simplicity

        // Transfer SOL from the buyer to the presale account
        invoke(
            &system_instruction::transfer(
                &ctx.accounts.buyer.key,
                &ctx.accounts.presale_account.key(),
                sol_amount,
            ),
            &[
                ctx.accounts.buyer.to_account_info(),
                ctx.accounts.presale_account.to_account_info(),
            ],
        )?;

        // Token transfer logic remains the same
        // (Add your existing logic for transferring tokens from the presale's token account to the buyer's token account)

        Ok(())
    }

    pub fn end_presale(ctx: Context<EndPresale>) -> Result<()> {
        let presale_account = &mut ctx.accounts.presale_account;
        presale_account.is_active = false;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8 + 8 + 8 + 1)]
    pub presale_account: Account<'info, PresaleAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BuyTokens<'info> {
    #[account(mut)]
    pub presale_account: Account<'info, PresaleAccount>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    // Include accounts for token transfer if necessary
    // e.g., presale_tokens, token_program, etc.
}

#[derive(Accounts)]
pub struct EndPresale<'info> {
    #[account(mut)]
    pub presale_account: Account<'info, PresaleAccount>,
}

#[account]
pub struct PresaleAccount {
    pub start_time: u64,
    pub end_time: u64,
    pub is_active: bool,
    // Consider including additional fields as necessary
    // e.g., total_sold, rate, etc.
}

#[error_code]
pub enum PresaleError {
    #[msg("The presale is not active.")]
    PresaleNotActive,
    #[msg("The presale has not started yet.")]
    PresaleNotStarted,
    #[msg("The presale has already ended.")]
    PresaleEnded,
    // Include additional error types as necessary
}
