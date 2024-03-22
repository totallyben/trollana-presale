use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;
use anchor_lang::solana_program::program::invoke;

use anchor_spl::{
    token_2022::{Token2022, ID as TOKEN_2022_ID, TransferChecked},
    token_interface::{Mint, TokenAccount},
    // associated_token::AssociatedToken,
};

declare_id!("GwrFvVJYaqPqoyDQvNYdr8a3ewvSTeyei92URkqU5Ak3"); // laptop
// declare_id!("CUF1pNp3pxjFUVQCvFpuAVhv6uXfutZhPe8sSDwmkyXF"); // office


#[program]
pub mod presale {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, _presale_ref: String, start_time: u64, end_time: u64, tokens_per_sol: f64, min_buy: f32, max_buy: f32, tokens_available: u64) -> Result<()> {
        msg!("Initialising");
        msg!("_presale_ref {}", _presale_ref);

        let presale_account = &mut ctx.accounts.presale_account;

        presale_account.is_initialized = true;
        presale_account.start_time = start_time;
        presale_account.end_time = end_time;
        presale_account.is_active = true;
        presale_account.destination_wallet_pubkey = *ctx.accounts.destination_wallet.key;
        presale_account.tokens_per_sol = tokens_per_sol;
        presale_account.min_buy = min_buy;
        presale_account.max_buy = max_buy;
        presale_account.tokens_available = tokens_available;
        presale_account.tokens_sold = 0;
        presale_account.amount_raised = 0.0;

        msg!("Initialising presale account");
        msg!("Start time {}", start_time);
        msg!("End time {}", end_time);
        msg!("Recipient wallet {}", presale_account.destination_wallet_pubkey);

        Ok(())
    }

    pub fn buy_tokens(ctx: Context<BuyTokens>, presale_ref: String, _buyer_ref: String, sol_lamports_amount: u64) -> Result<()> {
        msg!("BuyTokens");
        msg!("presale_ref {}", presale_ref);

        let presale_account = &mut ctx.accounts.presale_account;

        // Check if the presale account has been initialized
        require!(presale_account.is_initialized, PresaleError::NotInitialized);

        let clock = Clock::get().unwrap();

        // Check if the presale is active and within the time bounds
        require!(presale_account.is_active, PresaleError::PresaleNotActive);
        require!(clock.unix_timestamp as u64 >= presale_account.start_time, PresaleError::PresaleNotStarted);
        require!(clock.unix_timestamp as u64 <= presale_account.end_time, PresaleError::PresaleEnded);

        let buyer = &ctx.accounts.buyer;
        let destination_wallet = &ctx.accounts.destination_wallet;

        // destination wallet must match
        require!(destination_wallet.key.to_string() == presale_account.destination_wallet_pubkey.to_string(), PresaleError::InvalidDestinationWallet);

        let sol_amount = lamports_to_sol(sol_lamports_amount);

        // check valid sol amount
        require!(sol_amount >= presale_account.min_buy, PresaleError::BuyAmountTooLow);
        require!(sol_amount <= presale_account.max_buy, PresaleError::BuyAmountTooHigh);

        // update buyer account
        let buyer_account = &mut ctx.accounts.buyer_account;

        msg!("buyer_account.total_spent before {}", buyer_account.total_spent);
        buyer_account.total_spent += sol_amount;
        require!(buyer_account.total_spent <= presale_account.max_buy, PresaleError::BuyAmountTooHigh);
        msg!("buyer_account.total_spent after {}", buyer_account.total_spent);
        msg!("presale_account.max_buy {}", presale_account.max_buy);

        // Create a transfer instruction from the buyer to the destination wallet
        let transfer_instruction = system_instruction::transfer(
            &buyer.key(),
            &destination_wallet.key(),
            sol_lamports_amount,
        );

        // Invoke the transfer instruction
        msg!("Initiating transfer of {} SOL to recipient wallet", sol_amount);
        invoke(
            &transfer_instruction,
            &[
                buyer.to_account_info().clone(),
                destination_wallet.to_account_info().clone(),
                ctx.accounts.system_program.to_account_info().clone(),
            ],
        )?;
        msg!("Completed transfer of {} SOL from recipient wallet", sol_amount);

        let token_amount = sol_to_token(sol_amount, presale_account.tokens_per_sol, 9).ok_or(PresaleError::OverflowError)?;

        let seeds = &[presale_ref.as_bytes(), b"token_account_authority".as_ref()];
        let (_, bump_seed) = Pubkey::find_program_address(seeds, ctx.program_id);

        let new_seeds = &[
            presale_ref.as_bytes(),
            b"token_account_authority".as_ref(),
            &[bump_seed],
        ];
        let signer_seeds = &[&new_seeds[..]];

        msg!("Token account key: {}", ctx.accounts.token_account.to_account_info().key);
        msg!("Token account balance: {}", ctx.accounts.token_account.amount);

        msg!("Initiating transfer of {} tokens to buyer wallet", token_amount);
        anchor_spl::token_2022::transfer_checked(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.token_account.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.buyer_token_account.to_account_info(),
                    authority: ctx.accounts.token_account_authority.to_account_info(),
                },
                signer_seeds,
            ), 
            token_amount,
            9,
        )?;
        msg!("Completed transfer of {} tokens to buyer wallet", token_amount);

        presale_account.tokens_sold += token_amount_without_decimal(token_amount, 9);
        presale_account.amount_raised += sol_amount;

        Ok(())
    }

    pub fn end_presale(ctx: Context<EndPresale>) -> Result<()> {
        let presale_account = &mut ctx.accounts.presale_account;
        presale_account.is_active = false;
        Ok(())
    }
}

pub fn lamports_to_sol(lamports: u64) -> f32 {
    lamports as f32 / 1_000_000_000.0
}

fn sol_to_token(sol_amount: f32, tokens_per_sol: f64, decimal_places: u32) -> Option<u64> {
    let multiplier = 10u64.pow(decimal_places);
    let token_amount = (sol_amount as f64 * tokens_per_sol).round() as u64;
    token_amount.checked_mul(multiplier)
}

fn token_amount_without_decimal(token_amount: u64, decimal_places: u32) -> u64 {
    let divisor = 10u64.pow(decimal_places);
    return token_amount / divisor;
}

#[derive(Accounts)]
#[instruction(presale_ref: String)] 
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [presale_ref.as_bytes(), b"presale_account".as_ref()], 
        bump,
        payer = payer,
        space = 98 + 16,
    )]
    pub presale_account: Account<'info, PresaleAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        seeds = [presale_ref.as_bytes(), b"token_account".as_ref()], 
        bump,
        payer = payer, 
        token::mint = mint, 
        token::authority = token_account_authority,
        token::token_program = token_program,
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    /// CHECK: This account is only used to authorize transactions from the presale_account
    #[account(
        seeds = [presale_ref.as_bytes(), b"token_account_authority".as_ref()], 
        bump,
    )]
    pub token_account_authority: AccountInfo<'info>,

    #[account(
        mint::token_program = TOKEN_2022_ID,
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    /// CHECK: This account is only used to derive its pubkey
    #[account()]
    pub destination_wallet: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    #[account(address = TOKEN_2022_ID)]
    pub token_program: Program<'info, Token2022>,
}

#[derive(Accounts)]
#[instruction(presale_ref: String, buyer_ref: String)] 
pub struct BuyTokens<'info> {
    #[account(mut)]
    pub presale_account: Account<'info, PresaleAccount>,
    #[account(mut)]
    pub buyer: Signer<'info>,

    /// CHECK: This is used to store the amount of 
    #[account(
        init_if_needed,
        seeds = [presale_ref.as_bytes(), buyer_ref.as_bytes(), b"buyer_account".as_ref()], 
        bump,
        payer = buyer,
        space = 8 + 32 + 8 + 8
    )]
    pub buyer_account: Account<'info, BuyerAccount>,

    /// CHECK: This account is only used to send tokens to the buyer
    #[account(mut)]
    pub buyer_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mint::token_program = TOKEN_2022_ID,
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    /// CHECK: This account is only used to send tokens to the buyer
    #[account(mut)]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    /// CHECK: This account is used to as the authority on the 
    #[account(mut)]
    pub token_account_authority: AccountInfo<'info>,
    /// CHECK: This account is only used to send SOL to
    #[account(mut)]
    pub destination_wallet: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    #[account(address = TOKEN_2022_ID)]
    pub token_program: Program<'info, Token2022>,
    // #[account(address = AssociatedToken::id())]
    // pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct EndPresale<'info> {
    #[account(mut)]
    pub presale_account: Account<'info, PresaleAccount>,
}

#[account]
pub struct PresaleAccount {
    pub is_initialized: bool,
    pub start_time: u64,
    pub end_time: u64,
    pub is_active: bool,
    pub destination_wallet_pubkey: Pubkey,
    pub tokens_per_sol: f64,
    pub min_buy: f32,
    pub max_buy: f32,
    pub tokens_available: u64,
    pub tokens_sold: u64,
    pub amount_raised: f32,
}

#[account]
pub struct BuyerAccount {
    pub buyer_pubkey: Pubkey,
    pub total_spent: f32,
}

#[error_code]
pub enum PresaleError {
    #[msg("The presale account has not initialized yet.")]
    NotInitialized,
    #[msg("The presale is not active.")]
    PresaleNotActive,
    #[msg("The presale has not started yet.")]
    PresaleNotStarted,
    #[msg("The presale has already ended.")]
    PresaleEnded,
    #[msg("Could not calculate the correct amount of tokens.")]
    OverflowError,
    #[msg("Purchase amount to low.")]
    BuyAmountTooLow,
    #[msg("Maximum purchase amount exceeded.")]
    BuyAmountTooHigh,
    #[msg("Invalid destination wallet.")]
    InvalidDestinationWallet,
    // Include additional error types as necessary
}