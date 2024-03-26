use anchor_lang::prelude::*;

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

#[account]
pub struct PurchaseReceipt {
    pub buyer_ref: String,
    pub sol_lamports_amount: u64,
    pub tokens_purchased: u64,
}

#[account]
pub struct ProceedsVault {}