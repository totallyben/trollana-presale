use anchor_lang::prelude::*;
use anchor_spl::associated_token;
use anchor_lang::solana_program::{
    program::invoke,
    // program::invoke_signed,
    system_instruction,
};
use anchor_spl::{
    token_2022::{Token2022, ID as TOKEN_2022_ID, TransferChecked},
    token_interface::{Mint, TokenAccount},
    associated_token::{AssociatedToken, Create},
};

use crate::errors::PresaleError;
use crate::account::*;

#[derive(Accounts)]
#[instruction(presale_ref: String)] 
pub struct EndPresale<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub presale_account: Account<'info, PresaleAccount>,
    /// CHECK: This account is only used to send unsold tokens to
    #[account(mut)]
    pub recipient_wallet: AccountInfo<'info>,
    /// CHECK: This account is only used to send tokens to the buyer
    #[account(mut)]
    pub recipient_wallet_token_account: UncheckedAccount<'info>,
    /// CHECK: This account is only used to send tokens to the buyer
    #[account(mut)]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    /// CHECK: This account is used to as the authority on the 
    #[account(mut)]
    pub token_account_authority: AccountInfo<'info>,
    #[account(address = TOKEN_2022_ID)]

    /// CHECK: This account is used to store the presale proceeds
    #[account(
        seeds = [presale_ref.as_bytes(), b"proceeds_vault".as_ref()], 
        bump,
    )]
    pub proceeds_vault: AccountInfo<'info>,

    pub token_program: Program<'info, Token2022>,
    #[account(
        mint::token_program = TOKEN_2022_ID,
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn end_presale(
    ctx: Context<EndPresale>,
    presale_ref: String,
) -> Result<()> {
    msg!("end presale {}", presale_ref);
    let presale_account = &mut ctx.accounts.presale_account;

    require!(ctx.accounts.payer.key() == presale_account.owner.key(), PresaleError::IllegalOwner);
    require!(presale_account.is_active, PresaleError::PresaleNotActive);
    require!(presale_account.tokens_distributed, PresaleError::TokensNotDistributed);

    let proceeds_vault = &mut ctx.accounts.proceeds_vault;
    // let recipient_wallet = &mut ctx.accounts.recipient_wallet;
    let lamports_amount = proceeds_vault.lamports();

    if lamports_amount > 0 {
        // // transfer sol from vault to destination wallet
        // let transfer_instruction = system_instruction::transfer(
        //     &proceeds_vault.key(),
        //     &recipient_wallet.key(),
        //     lamports_amount,
        // );

        // invoke(
        //     &transfer_instruction,
        //     &[
        //         proceeds_vault.to_account_info().clone(),
        //         recipient_wallet.to_account_info().clone(),
        //         ctx.accounts.system_program.to_account_info().clone(),
        //     ],
        // )?;
        msg!("completed transfer of {} SOL to recipient wallet", presale_account.amount_raised);
    } else {
        msg!("no SOL to transfter to recipient wallet");
    }

    // // create associated account to store tokens in recipient wallet
    // if ctx.accounts.recipient_wallet_token_account.to_account_info().data_len() == 0 {
    //     let cpi_accounts = Create {
    //         payer: ctx.accounts.payer.to_account_info(),
    //         associated_token: ctx.accounts.recipient_wallet_token_account.to_account_info(),
    //         authority: ctx.accounts.recipient_wallet.to_account_info(),
    //         mint: ctx.accounts.mint.to_account_info(),
    //         system_program: ctx.accounts.system_program.to_account_info(),
    //         token_program: ctx.accounts.token_program.to_account_info(),
    //     };

    //     let cpi_program = ctx.accounts.associated_token_program.to_account_info();

    //     let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    //     let _ = associated_token::create(cpi_ctx);
    // }

    // let mut tokens_remaining = presale_account.tokens_available - presale_account.tokens_sold;
    // let multiplier = 10u64.pow(9);
    // tokens_remaining = tokens_remaining.checked_mul(multiplier).expect("Overflow occurred");

    // let seeds = &[presale_ref.as_bytes(), b"token_account_authority".as_ref()];
    // let (_, bump_seed) = Pubkey::find_program_address(seeds, ctx.program_id);

    // let new_seeds = &[
    //     presale_ref.as_bytes(),
    //     b"token_account_authority".as_ref(),
    //     &[bump_seed],
    // ];
    // let signer_seeds = &[&new_seeds[..]];

    // msg!("initiating transfer of remaining {} tokens to recipient wallet", tokens_remaining);
    // anchor_spl::token_2022::transfer_checked(
    //     CpiContext::new_with_signer(
    //         ctx.accounts.token_program.to_account_info(),
    //         TransferChecked {
    //             from: ctx.accounts.token_account.to_account_info(),
    //             mint: ctx.accounts.mint.to_account_info(),
    //             to: ctx.accounts.recipient_wallet_token_account.to_account_info(),
    //             authority: ctx.accounts.token_account_authority.to_account_info(),
    //         },
    //         signer_seeds,
    //     ), 
    //     tokens_remaining,
    //     9,
    // )?;
    // msg!("completed transfer of {} tokens to recipient wallet", tokens_remaining);

    // presale_account.is_active = false;
    // msg!("presale {} no longer active", presale_ref);

    Ok(())
}