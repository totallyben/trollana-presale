use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    program::invoke,
    // program::invoke_signed,
    system_instruction,
};

use crate::errors::PresaleError;
use crate::account::{PresaleAccount, BuyerAccount};
use crate::util::*;

#[derive(Accounts)]
#[instruction(presale_ref: String, buyer_ref: String)] 
pub struct BuyTokens<'info> {
    #[account(mut)]
    pub presale_account: Account<'info, PresaleAccount>,
    #[account(mut)]
    pub buyer: Signer<'info>,

    /// CHECK: This is used to store the amount of sol spent so far
    #[account(
        init_if_needed,
        seeds = [presale_ref.as_bytes(), buyer_ref.as_bytes(), b"buyer_account".as_ref()], 
        bump,
        payer = buyer,
        space = 8 + 32 + 8 + 8
    )]
    pub buyer_account: Account<'info, BuyerAccount>,

    /// CHECK: This account is used to store the presale proceeds
    #[account(mut)]
    pub proceeds_vault: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn buy(
    ctx: Context<BuyTokens>, 
    _presale_ref: String, 
    _buyer_ref: String, 
    sol_lamports_amount: u64
) -> Result<()> {
    let presale_account = &mut ctx.accounts.presale_account;

    // Check if the presale account has been initialized
    require!(presale_account.is_initialized, PresaleError::NotInitialized);

    let clock = Clock::get().unwrap();

    // Check if the presale is active and within the time bounds
    require!(presale_account.is_active, PresaleError::PresaleNotActive);
    require!(clock.unix_timestamp as u64 >= presale_account.start_time, PresaleError::PresaleNotStarted);
    require!(clock.unix_timestamp as u64 <= presale_account.end_time, PresaleError::PresaleEnded);

    let buyer = &ctx.accounts.buyer;
    let sol_amount = lamports_to_sol(sol_lamports_amount);
    let tokens_purchased = sol_to_token(sol_amount, presale_account.tokens_per_sol, 9).ok_or(PresaleError::OverflowError)?;

    // check valid sol amount
    require!(sol_amount >= presale_account.min_buy, PresaleError::BuyAmountTooLow);
    require!(sol_amount <= presale_account.max_buy, PresaleError::BuyAmountTooHigh);

    // update buyer account
    let buyer_account = &mut ctx.accounts.buyer_account;
    let proceeds_vault = &mut ctx.accounts.proceeds_vault;

    // msg!("buyer_account.total_spent before {}", buyer_account.total_spent);
    buyer_account.total_spend += sol_amount;
    buyer_account.tokens_purchased = tokens_purchased;

    require!(buyer_account.total_spend <= presale_account.max_buy, PresaleError::BuyAmountTooHigh);
    // msg!("buyer_account.total_spend after {}", buyer_account.total_spent);
    // msg!("presale_account.max_buy {}", presale_account.max_buy);

    // Create a transfer instruction from the buyer to the proceeds vault
    let transfer_instruction = system_instruction::transfer(
        &buyer.key(),
        &proceeds_vault.key(),
        sol_lamports_amount,
    );

    // Invoke the transfer instruction
    invoke(
        &transfer_instruction,
        &[
            buyer.to_account_info().clone(),
            proceeds_vault.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info().clone(),
        ],
    )?;
    msg!("transferred {} SOL to proceeds vault", sol_amount);

    let token_amount_without_decimal = token_amount_without_decimal(tokens_purchased, 9);

    presale_account.tokens_sold += token_amount_without_decimal;
    presale_account.amount_raised += sol_amount;
    presale_account.num_sales += 1;
    msg!("receipt: token={}, buyer={}, spend={}, tokens={}", presale_account.token_mint_address.key().to_string(), buyer.key().to_string(), sol_amount, token_amount_without_decimal);
    msg!("presaleInfo: token={}, sales={}, amountRaised={}", presale_account.token_mint_address.key().to_string(), presale_account.num_sales, presale_account.amount_raised);

    Ok(())
}