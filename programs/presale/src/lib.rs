use anchor_lang::prelude::*;

pub mod account;
pub use account::*;

pub mod util;
pub mod errors;

pub mod initialize;
pub use initialize::*;

pub mod purchase;
pub use purchase::*;

pub mod distribute;
pub use distribute::*;

pub mod end;
pub use end::*;

declare_id!("AtFh34ewcFEv5KuuBcd6iNrEp7ok9n1JM5rVeWVBPovs");

#[program]
pub mod presale {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        presale_ref: String,
        start_time: u64,
        end_time: u64,
        tokens_per_sol: f64,
        fee_percent: f32,
        min_buy: f32,
        max_buy: f32,
        tokens_available: u64,
    ) -> Result<()> {    
        initialize::init(
            ctx,
            presale_ref,
            start_time,
            end_time,
            tokens_per_sol,
            fee_percent,
            min_buy,
            max_buy,
            tokens_available,
        )
    }

    pub fn buy_tokens(
        ctx: Context<BuyTokens>, 
        presale_ref: String, 
        buyer_ref: String, 
        sol_lamports_amount: u64
    ) -> Result<()> {
        purchase::buy(
            ctx,
            presale_ref,
            buyer_ref,
            sol_lamports_amount,
        )
    }

    pub fn complete_dist(
        ctx: Context<CompleteDistribution>,
    ) -> Result<()> {
        distribute::complete_distribion(ctx)
    }

    pub fn end(
        ctx: Context<EndPresale>,
        presale_ref: String,
    ) -> Result<()> {
        end::end_presale(ctx, presale_ref)
    }

    // pub fn buy_tokens(ctx: Context<BuyTokens>, presale_ref: String, buyer_ref: String, sol_lamports_amount: u64) -> Result<()> {
    //     msg!("buy tokens {}", presale_ref);

    //     let presale_account = &mut ctx.accounts.presale_account;

    //     // Check if the presale account has been initialized
    //     require!(presale_account.is_initialized, PresaleError::NotInitialized);

    //     let clock = Clock::get().unwrap();

    //     // Check if the presale is active and within the time bounds
    //     require!(presale_account.is_active, PresaleError::PresaleNotActive);
    //     require!(clock.unix_timestamp as u64 >= presale_account.start_time, PresaleError::PresaleNotStarted);
    //     require!(clock.unix_timestamp as u64 <= presale_account.end_time, PresaleError::PresaleEnded);

    //     let buyer = &ctx.accounts.buyer;
    //     let sol_amount = lamports_to_sol(sol_lamports_amount);

    //     // check valid sol amount
    //     require!(sol_amount >= presale_account.min_buy, PresaleError::BuyAmountTooLow);
    //     require!(sol_amount <= presale_account.max_buy, PresaleError::BuyAmountTooHigh);

    //     // update buyer account
    //     let buyer_account = &mut ctx.accounts.buyer_account;

    //     // msg!("buyer_account.total_spent before {}", buyer_account.total_spent);
    //     buyer_account.total_spent += sol_amount;
    //     require!(buyer_account.total_spent <= presale_account.max_buy, PresaleError::BuyAmountTooHigh);
    //     // msg!("buyer_account.total_spent after {}", buyer_account.total_spent);
    //     // msg!("presale_account.max_buy {}", presale_account.max_buy);

    //     let proceeds_vault = &mut ctx.accounts.proceeds_vault;
    //     // let purchase_receipt = &mut create_purchase_receipt_account(&ctx, &presale_ref, presale_account.num_sales)?;
    //     let purchase_receipt = &mut ctx.accounts.purchase_receipt;

    //     // Create a transfer instruction from the buyer to the proceeds vault
    //     let transfer_instruction = system_instruction::transfer(
    //         &buyer.key(),
    //         &proceeds_vault.key(),
    //         sol_lamports_amount,
    //     );

    //     // Invoke the transfer instruction
    //     msg!("initiating transfer of {} SOL to proceeds vault", sol_amount);
    //     invoke(
    //         &transfer_instruction,
    //         &[
    //             buyer.to_account_info().clone(),
    //             proceeds_vault.to_account_info().clone(),
    //             ctx.accounts.system_program.to_account_info().clone(),
    //         ],
    //     )?;
    //     msg!("completed transfer of {} SOL to proceeds vault", sol_amount);

    //     let token_amount = sol_to_token(sol_amount, presale_account.tokens_per_sol, 9).ok_or(PresaleError::OverflowError)?;

    //     // let seeds = &[presale_ref.as_bytes(), b"token_account_authority".as_ref()];
    //     // let (_, bump_seed) = Pubkey::find_program_address(seeds, ctx.program_id);

    //     // let new_seeds = &[
    //     //     presale_ref.as_bytes(),
    //     //     b"token_account_authority".as_ref(),
    //     //     &[bump_seed],
    //     // ];
    //     // let signer_seeds = &[&new_seeds[..]];

    //     // msg!("Token account key: {}", ctx.accounts.token_account.to_account_info().key);
    //     // msg!("Token account balance: {}", ctx.accounts.token_account.amount);

    //     // msg!("initiating transfer of {} tokens to buyer wallet", token_amount);
    //     // anchor_spl::token_2022::transfer_checked(
    //     //     CpiContext::new_with_signer(
    //     //         ctx.accounts.token_program.to_account_info(),
    //     //         TransferChecked {
    //     //             from: ctx.accounts.token_account.to_account_info(),
    //     //             mint: ctx.accounts.mint.to_account_info(),
    //     //             to: ctx.accounts.buyer_token_account.to_account_info(),
    //     //             authority: ctx.accounts.token_account_authority.to_account_info(),
    //     //         },
    //     //         signer_seeds,
    //     //     ), 
    //     //     token_amount,
    //     //     9,
    //     // )?;
    //     // msg!("completed transfer of {} tokens to buyer wallet", token_amount);

    //     presale_account.tokens_sold += token_amount_without_decimal(token_amount, 9);
    //     presale_account.amount_raised += sol_amount;
    //     presale_account.num_sales += 1;

    //     purchase_receipt.buyer_ref = buyer_ref;
    //     purchase_receipt.sol_lamports_amount = sol_lamports_amount;
    //     purchase_receipt.tokens_purchased = token_amount;

    //     Ok(())
    // }
}


// #[derive(Accounts)]
// #[instruction(presale_ref: String, buyer_ref: String)] 
// pub struct BuyTokens<'info> {
//     #[account(mut)]
//     pub presale_account: Account<'info, PresaleAccount>,
//     #[account(mut)]
//     pub buyer: Signer<'info>,

//     /// CHECK: This is used to store the amount of sol spent so far
//     #[account(
//         init_if_needed,
//         seeds = [presale_ref.as_bytes(), buyer_ref.as_bytes(), b"buyer_account".as_ref()], 
//         bump,
//         payer = buyer,
//         space = 8 + 32 + 8 + 8
//     )]
//     pub buyer_account: Account<'info, BuyerAccount>,

//     /// CHECK: This is used to store the amount of sol spent so far
//     #[account(
//         init_if_needed,
//         seeds = [presale_ref.as_bytes(), &presale_account.num_sales.to_string().as_bytes(), b"purchase_receipt".as_ref()], 
//         bump,
//         payer = buyer,
//         space = 14 + 8 + 8
//     )]
//     pub purchase_receipt: Account<'info, PurchaseReceipt>,

//     /// CHECK: This account is used to store the presale proceeds
//     #[account(mut)]
//     pub proceeds_vault: AccountInfo<'info>,

//     // /// CHECK: This account is only used to send tokens to the buyer
//     // #[account(mut)]
//     // pub buyer_token_account: UncheckedAccount<'info>,

//     #[account(
//         mint::token_program = TOKEN_2022_ID,
//     )]
//     pub mint: InterfaceAccount<'info, Mint>,

//     /// CHECK: This account is only used to send tokens to the buyer
//     #[account(mut)]
//     pub token_account: InterfaceAccount<'info, TokenAccount>,
//     /// CHECK: This account is only used to authorize transactions from the token_account
//     #[account(mut)]
//     pub token_account_authority: AccountInfo<'info>,
//     // /// CHECK: This account is only used to send SOL to
//     // #[account(mut)]
//     // pub recipient_wallet: AccountInfo<'info>,
//     pub system_program: Program<'info, System>,
//     #[account(address = TOKEN_2022_ID)]
//     pub token_program: Program<'info, Token2022>,
//     pub associated_token_program: Program<'info, AssociatedToken>,
//     pub rent: Sysvar<'info, Rent>,
//     // #[account(address = AssociatedToken::id())]
//     // pub associated_token_program: Program<'info, AssociatedToken>,
// }
