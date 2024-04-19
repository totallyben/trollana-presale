pub fn lamports_to_sol(lamports: u64) -> f32 {
    lamports as f32 / 1_000_000_000.0
}

pub fn sol_to_token(sol_amount: f32, tokens_per_sol: f64, decimal_places: u32) -> Option<u64> {
    let multiplier = 10u64.pow(decimal_places);
    let token_amount = (sol_amount as f64 * tokens_per_sol).round() as u64;
    token_amount.checked_mul(multiplier)
}

pub fn token_amount_without_decimal(token_amount: u64, decimal_places: u32) -> u64 {
    let divisor = 10u64.pow(decimal_places);
    return token_amount / divisor;
}