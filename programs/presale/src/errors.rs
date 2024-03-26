use anchor_lang::prelude::*;

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
    #[msg("Tokens have not been distributed yet.")]
    TokensNotDistributed,
    #[msg("Illegal owner.")]
    IllegalOwner,
    // Include additional error types as necessary
}