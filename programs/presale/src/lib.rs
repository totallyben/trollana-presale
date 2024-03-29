use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;
use anchor_lang::solana_program::program::invoke;

use anchor_spl::{
    token_2022::{Token2022, ID as TOKEN_2022_ID, TransferChecked},
    token_interface::{Mint, TokenAccount},
    associated_token::{AssociatedToken, Create},
};

declare_id!("4MFUeSPBzux3EqM6Ltv27o3YUbyAhwTnWLE4QRnBVyYH");

#[program]
pub mod presale {
    use anchor_spl::associated_token;

    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        _presale_ref: String,
        start_time: u64,
        end_time: u64,
        tokens_per_sol: f64,
        fee_percent: f32,
        min_buy: f32,
        max_buy: f32,
        tokens_available: u64,
    ) -> Result<()> {    
        msg!("initialising presale {}", _presale_ref);
        let presale_account = &mut ctx.accounts.presale_account;
    
        presale_account.is_initialized = true;
        presale_account.owner = ctx.accounts.payer.key();
        presale_account.token_mint_address = ctx.accounts.mint.key();
        presale_account.start_time = start_time;
        presale_account.end_time = end_time;
        presale_account.is_active = true;
        presale_account.recipient_wallet = *ctx.accounts.recipient_wallet.key;
        presale_account.tokens_per_sol = tokens_per_sol;
        presale_account.fee_percent = fee_percent;
        presale_account.min_buy = min_buy;
        presale_account.max_buy = max_buy;
        presale_account.tokens_available = tokens_available;
        presale_account.tokens_sold = 0;
        presale_account.amount_raised = 0.0;
        presale_account.num_sales = 0;
        presale_account.tokens_distributed = false;
    
        // let buyer_registry = &mut ctx.accounts.buyer_registry;
        // buyer_registry.buyers = Vec::new();
    
        Ok(())
    }

    pub fn buy_tokens(ctx: Context<BuyTokens>, _presale_ref: String, _buyer_ref: String, sol_lamports_amount: u64) -> Result<()> {
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
        buyer_account.total_spend += sol_amount;
        buyer_account.tokens_purchased = tokens_purchased;


        let destination_wallet = &ctx.accounts.destination_wallet;

        // destination wallet must match
        require!(destination_wallet.key.to_string() == presale_account.recipient_wallet.to_string(), PresaleError::InvalidDestinationWallet);

        // Create a transfer instruction from the buyer to the destination wallet
        let transfer_instruction = system_instruction::transfer(
            &buyer.key(),
            &destination_wallet.key(),
            sol_lamports_amount,
        );

        // Invoke the transfer instruction
        // msg!("Initiating transfer of {} SOL to recipient wallet", sol_amount);
        invoke(
            &transfer_instruction,
            &[
                buyer.to_account_info().clone(),
                destination_wallet.to_account_info().clone(),
                ctx.accounts.system_program.to_account_info().clone(),
            ],
        )?;
        msg!("Completed transfer of {} SOL from recipient wallet", sol_amount);

        let token_amount_without_decimal = token_amount_without_decimal(tokens_purchased, 9);

        presale_account.tokens_sold += token_amount_without_decimal;
        presale_account.amount_raised += sol_amount;
        presale_account.num_sales += 1;
        msg!("receipt: token={}, buyer={}, spend={}, tokens={}", presale_account.token_mint_address.key().to_string(), buyer.key().to_string(), sol_amount, token_amount_without_decimal);
        msg!("presaleInfo: token={}, sales={}, amountRaised={}", presale_account.token_mint_address.key().to_string(), presale_account.num_sales, presale_account.amount_raised);

        Ok(())
    }

    pub fn end_presale(ctx: Context<EndPresale>, presale_ref: String) -> Result<()> {
        msg!("end presale {}", presale_ref);
        let presale_account = &mut ctx.accounts.presale_account;

        require!(presale_account.is_active, PresaleError::PresaleNotActive);

        if ctx.accounts.destination_wallet_token_account.to_account_info().data_len() == 0 {
            let cpi_accounts = Create {
                payer: ctx.accounts.payer.to_account_info(),
                associated_token: ctx.accounts.destination_wallet_token_account.to_account_info(),
                authority: ctx.accounts.destination_wallet.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
            };

            let cpi_program = ctx.accounts.associated_token_program.to_account_info();

            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
            let _ = associated_token::create(cpi_ctx);
        }

        let mut tokens_remaining = presale_account.tokens_available - presale_account.tokens_sold;
        let multiplier = 10u64.pow(9);
        tokens_remaining = tokens_remaining.checked_mul(multiplier).expect("Overflow occurred");

        let seeds = &[presale_ref.as_bytes(), b"token_account_authority".as_ref()];
        let (_, bump_seed) = Pubkey::find_program_address(seeds, ctx.program_id);

        let new_seeds = &[
            presale_ref.as_bytes(),
            b"token_account_authority".as_ref(),
            &[bump_seed],
        ];
        let signer_seeds = &[&new_seeds[..]];

        msg!("initiating transfer of remaining {} tokens to recipient wallet", tokens_remaining);
        anchor_spl::token_2022::transfer_checked(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.token_account.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.destination_wallet_token_account.to_account_info(),
                    authority: ctx.accounts.token_account_authority.to_account_info(),
                },
                signer_seeds,
            ), 
            tokens_remaining,
            9,
        )?;
        msg!("completed transfer of {} tokens to recipient wallet", tokens_remaining);

        presale_account.is_active = false;

        msg!("presale {} no longer active", presale_ref);

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
        space = 196,
    )]
    pub presale_account: Box<Account<'info, PresaleAccount>>,
    
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: This account is used to store the presale proceeds
    #[account(
        seeds = [presale_ref.as_bytes(), b"proceeds_vault".as_ref()], 
        bump,
    )]
    pub proceeds_vault: AccountInfo<'info>,

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

    /// CHECK: This account is only used to authorize transactions from the token_account
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
    pub recipient_wallet: AccountInfo<'info>,
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

    /// CHECK: This is used to store the amount of sol spent so far
    #[account(
        init_if_needed,
        seeds = [presale_ref.as_bytes(), buyer_ref.as_bytes(), b"buyer_account".as_ref()], 
        bump,
        payer = buyer,
        space = 8 + 32 + 8 + 8
    )]
    pub buyer_account: Account<'info, BuyerAccount>,

    /// CHECK: This account is only used to send SOL to
    #[account(mut)]
    pub destination_wallet: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EndPresale<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub presale_account: Account<'info, PresaleAccount>,
    /// CHECK: This account is only used to send unsold tokens to
    #[account(mut)]
    pub destination_wallet: AccountInfo<'info>,
    /// CHECK: This account is only used to send tokens to the buyer
    #[account(mut)]
    pub destination_wallet_token_account: UncheckedAccount<'info>,
    /// CHECK: This account is only used to send tokens to the buyer
    #[account(mut)]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    /// CHECK: This account is used to as the authority on the 
    #[account(mut)]
    pub token_account_authority: AccountInfo<'info>,
    #[account(address = TOKEN_2022_ID)]
    pub token_program: Program<'info, Token2022>,
    #[account(
        mint::token_program = TOKEN_2022_ID,
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[account]
pub struct PresaleAccount {
    pub is_initialized: bool,
    pub owner: Pubkey,
    pub token_mint_address: Pubkey,
    pub start_time: u64,
    pub end_time: u64,
    pub is_active: bool,
    pub recipient_wallet: Pubkey,
    pub tokens_per_sol: f64,
    pub fee_percent: f32,
    pub min_buy: f32,
    pub max_buy: f32,
    pub tokens_available: u64,
    pub tokens_sold: u64,
    pub amount_raised: f32,
    pub num_sales: u32,
    pub tokens_distributed: bool,
}

#[account]
pub struct BuyerAccount {
    pub buyer_pubkey: Pubkey,
    pub total_spend: f32,
    pub tokens_purchased: u64,
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