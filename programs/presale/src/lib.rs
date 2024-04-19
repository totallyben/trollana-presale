use anchor_lang::prelude::*;

pub mod account;
pub use account::*;

pub mod util;
pub mod errors;

pub mod initialize;
pub use initialize::*;

pub mod update;
pub use update::*;

pub mod purchase;
pub use purchase::*;

// pub mod distribute;
// pub use distribute::*;

pub mod end;
pub use end::*;

declare_id!("7ovz8eTEuHtXsURXevnRmsmim64STrfQVGxdvSszWJZR");

#[program]
pub mod presale {
    // use anchor_spl::associated_token;

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

    pub fn update_start_end(
        ctx: Context<UpdateStartEnd>,
        start_time: u64,
        end_time: u64,
    ) -> Result<()> {    
        update::update(
            ctx,
            start_time,
            end_time,
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

    pub fn end(
        ctx: Context<EndPresale>,
        presale_ref: String,
    ) -> Result<()> {
        end::end_presale(ctx, presale_ref)
    }
}