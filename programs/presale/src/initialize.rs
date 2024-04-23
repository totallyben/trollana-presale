use anchor_lang::prelude::*;
// use anchor_spl::token::{Mint, TokenAccount, Token};

use crate::account::*;

pub fn init(
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
    let presale_account = &mut ctx.accounts.presale_account;

    presale_account.is_initialized = true;
    presale_account.owner = ctx.accounts.payer.key();
    presale_account.token_mint_address = *ctx.accounts.token_mint_address.key;
    presale_account.start_time = start_time;
    presale_account.end_time = end_time;
    presale_account.is_active = true;
    presale_account.recipient_wallet = *ctx.accounts.recipient_wallet.key;
    presale_account.fee_wallet = *ctx.accounts.fee_wallet.key;
    presale_account.tokens_per_sol = tokens_per_sol;
    presale_account.fee_percent = fee_percent;
    presale_account.min_buy = min_buy;
    presale_account.max_buy = max_buy;
    presale_account.tokens_available = tokens_available;
    presale_account.tokens_sold = 0;
    presale_account.amount_raised = 0.0;
    // presale_account.num_sales = 0;
    // presale_account.tokens_distributed = false;

    // let buyer_registry = &mut ctx.accounts.buyer_registry;
    // buyer_registry.buyers = Vec::new();

    Ok(())
}

#[derive(Accounts)]
#[instruction(presale_ref: String)] 
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [presale_ref.as_bytes(), b"presale_account".as_ref()], 
        bump,
        payer = payer,
        space = 228,
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

    // #[account(
    //     init,
    //     seeds = [presale_ref.as_bytes(), b"token_account".as_ref()], 
    //     bump,
    //     payer = payer, 
    //     token::mint = mint, 
    //     token::authority = token_account_authority,
    //     token::token_program = token_program,
    // )]
    // pub token_account: InterfaceAccount<'info, TokenAccount>,

    // /// CHECK: This account is only used to authorize transactions from the token_account
    // #[account(
    //     seeds = [presale_ref.as_bytes(), b"token_account_authority".as_ref()], 
    //     bump,
    // )]
    // pub token_account_authority: AccountInfo<'info>,
    // #[account(
    //     owner = payer,
    //     mint::authority = mint,
    // )]
    // pub mint: InterfaceAccount<'info, Mint>,

    /// CHECK: This account is only used to derive its pubkey
    #[account()]
    pub token_mint_address: AccountInfo<'info>,

    /// CHECK: This account is only used to derive its pubkey
    #[account()]
    pub recipient_wallet: AccountInfo<'info>,
    /// CHECK: This account is only used to derive its pubkey
    #[account()]
    pub fee_wallet: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
    // pub token_program: Program<'info, Token>,
}