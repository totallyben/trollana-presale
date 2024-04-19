use anchor_lang::prelude::*;
// use anchor_spl::associated_token;
// use anchor_lang::solana_program::{
//     program::invoke,
//     // program::invoke_signed,
//     system_instruction,
// };
// use anchor_spl::{
//     token_2022::{Token2022, ID as TOKEN_2022_ID, TransferChecked},
//     token_interface::{Mint, TokenAccount},
//     associated_token::{AssociatedToken, Create},
// };

use crate::errors::PresaleError;
use crate::account::*;

pub fn end_presale(ctx: Context<EndPresale>, presale_ref: String) -> Result<()> {
    msg!("end presale {}", presale_ref);
    let presale_account = &mut ctx.accounts.presale_account;

    require!(ctx.accounts.payer.key() == presale_account.owner.key(), PresaleError::IllegalOwner);
    require!(presale_account.is_active, PresaleError::PresaleNotActive);

    presale_account.is_active = false;

    Ok(())
}

#[derive(Accounts)]
// #[instruction(presale_ref: String)]
pub struct EndPresale<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub presale_account: Account<'info, PresaleAccount>,
    // /// CHECK: This account is only used to send unsold tokens to
    // #[account(mut)]
    // pub recipient_wallet: AccountInfo<'info>,
    // /// CHECK: This account is only used to send tokens to the buyer
    // #[account(mut)]
    // pub recipient_wallet_token_account: UncheckedAccount<'info>,
    // /// CHECK: This account is only used to send tokens to the buyer
    // #[account(mut)]
    // pub token_account: InterfaceAccount<'info, TokenAccount>,
    // /// CHECK: This account is used to as the authority on the 
    // #[account(mut)]
    // pub token_account_authority: AccountInfo<'info>,
    // #[account(address = TOKEN_2022_ID)]

    // /// CHECK: This account is used to store the presale proceeds
    // #[account(
    //     seeds = [presale_ref.as_bytes(), b"proceeds_vault".as_ref()], 
    //     bump,
    // )]
    // pub proceeds_vault: AccountInfo<'info>,

    // pub token_program: Program<'info, Token2022>,
    // #[account(
    //     mint::token_program = TOKEN_2022_ID,
    // )]
    // pub mint: InterfaceAccount<'info, Mint>,
    // pub system_program: Program<'info, System>,
    // pub associated_token_program: Program<'info, AssociatedToken>,
}