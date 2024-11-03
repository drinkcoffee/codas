use eyre::{eyre, Result};
use uniswap_v3_sdk::prelude::FeeAmount;

pub fn fee_to_float(fee: FeeAmount) -> f32 {
    let fee_num: usize = fee as usize;
    let fee_num = fee_num as f32;
    fee_num / 10000.0
}

pub fn tick_to_exchange_rate(
    tick: i32,
    token_one_decimals: u64,
    token_two_decimals: u64,
) -> Result<f64> {
    let tick = tick as f64;
    let base: f64 = 1.0001;

    let token_decimals_diff = token_one_decimals.checked_sub(token_two_decimals);
    match token_decimals_diff {
        None => Err(eyre!(
            "Token decimals subtraction overflow: ({} - {})",
            token_one_decimals,
            token_two_decimals
        )),
        Some(diff) => {
            let denominator = 10u64.pow(diff.try_into()?);
            Ok(base.powf(tick) / denominator as f64)
        }
    }
}
