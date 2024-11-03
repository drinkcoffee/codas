use alloy::{
    eips::BlockId,
    primitives::{Address, ChainId},
};

//use uniswap_sdk_core::prelude::{CurrencyAmount, FractionBase};
use uniswap_v3_sdk::{
    extensions::{EphemeralTickDataProvider /* EphemeralTickMapDataProvider */},
    prelude::{FeeAmount, Pool, TickDataProvider},
    utils::compute_pool_address,
};

use eyre::Result;

use lib::prelude::*;

pub struct UniswapV3PoolSdk {
    pub pool: Pool,
    pub tick_data_provider: EphemeralTickDataProvider,
}

impl UniswapV3PoolSdk {
    pub async fn new(pool: Pool, tick_data_provider: EphemeralTickDataProvider) -> Result<Self> {
        Ok(Self {
            pool,
            tick_data_provider,
        })
    }

    pub async fn from_pool_key(
        chain_id: ChainId,
        factory: Address,
        token_a: Address,
        token_b: Address,
        fee: FeeAmount,
        provider: RootProvider,
        block_id: Option<BlockId>,
    ) -> Result<Self> {
        let pool_address = compute_pool_address(factory, token_a, token_b, fee, None, None);

        let tick_data_provider =
            EphemeralTickDataProvider::new(pool_address, &provider, None, None, block_id).await?;

        // let pool =
        //     Pool::from_pool_key_with_tick_data_provider(
        //         chain_id, factory, token_a, token_b, fee, provider, block_id).await?;
        let pool =
            Pool::from_pool_key(chain_id, factory, token_a, token_b, fee, provider, block_id)
                .await?;
        Self::new(pool, tick_data_provider).await
    }

    pub async fn info(&self) -> Result<i64> {
        println!("Current tick: {}", self.pool.tick_current);
        println!("Tick Spacing: {}", self.pool.tick_spacing());
        println!("Liquidity at current tick: {}", self.pool.liquidity);
        println!("Chain Id: {}", self.pool.chain_id());
        println!("Fee: Not sure how to use self.pool.fee" /* , self.pool.fee */);
        println!(
            "Token 0 symbol: {}",
            self.pool.token0.symbol.as_ref().unwrap()
        );
        println!(
            "Token 1 symbol: {}",
            self.pool.token1.symbol.as_ref().unwrap()
        );

        println!("get_input_amount fails without a tick data map provider");
        // let amount_to_swap = &CurrencyAmount::from_raw_amount(
        //     &(self.pool.token0), 98).unwrap();

        // let (input_amount, _) = self.pool.get_input_amount(
        //     amount_to_swap,
        //     None,
        // )
        // .unwrap();
        // println!("get_input_amount: 98 {} gives {} {}, which is hopefully also {}",
        //     self.pool.token0.symbol.as_ref().unwrap(),
        //     input_amount.quotient(),
        //     input_amount.currency.symbol.as_ref().unwrap(),
        //     self.pool.token1.symbol.as_ref().unwrap(),
        // );

        let hack = 0;
        Ok(hack)
    }

    // Some temporary code so that we can see what information is available.
    pub async fn dump(&self) -> Result<i64> {
        // let tick_map_data_provider = &self.pool.tick_data_provider;
        // println!("Tick lower: {}", tick_map_data_provider.tick_lower);
        // println!("Tick upper: {}", tick_map_data_provider.tick_upper);
        // println!("Tick spacing: {}", tick_map_data_provider.tick_spacing);
        // println!("Pool address: {}", tick_map_data_provider.pool);
        // println!("Tick map size: {}", tick_map_data_provider.bitmap.len());

        println!("Tick lower: {}", self.tick_data_provider.tick_lower);
        println!("Tick upper: {}", self.tick_data_provider.tick_upper);
        println!("Tick spacing: {}", self.tick_data_provider.tick_spacing);
        println!("Pool address: {}", self.tick_data_provider.pool);
        println!("Tick vector size: {}", self.tick_data_provider.ticks.len());

        let lower = self.tick_data_provider.tick_lower;
        let spacing = self.tick_data_provider.tick_spacing;
        // Note: This call result in error: Below smallest tick
        // let tick = self.tick_data_provider.get_tick(lower)?;
        // println!("Tick lower: index: {}", tick.index);
        // println!("Tick lower: liquidity gross: {}", tick.liquidity_gross);
        // println!("Tick lower: liquidity net: {}", tick.liquidity_net);

        let upper = self.tick_data_provider.tick_upper;
        // Note: This call result in error: Not contained in tick list
        // let tick = self.tick_data_provider.get_tick(upper)?;
        // println!("Tick upper: index: {}", tick.index);
        // println!("Tick upper: liquidity gross: {}", tick.liquidity_gross);
        // println!("Tick upper: liquidity net: {}", tick.liquidity_net);

        println!("Tick vector:");
        for tick in self.tick_data_provider.ticks.iter() {
            println!(
                "Tick: {}, liq gross: {}, liq net: {}",
                tick.index, tick.liquidity_gross, tick.liquidity_net
            );
        }

        // Note: no matter whether I pass in upper or lower, this errors with Below smallest tick
        let (tick_num, initialized) = self
            .tick_data_provider
            .next_initialized_tick_within_one_word(upper, true, spacing)?;
        println!("Next tick initialised: {}", initialized);
        println!("Next tick number: {}", tick_num);

        println!("Tick lower: {}", self.tick_data_provider.tick_lower);
        let hack = lower.as_i64();
        Ok(hack)
    }

    pub fn one_line_info(&self) -> Result<()> {
        let fee_num = crate::pool_calcs::fee_to_float(self.pool.fee);
        let rate = crate::pool_calcs::tick_to_exchange_rate(self.pool.tick_current, 18, 18)?;
        println!(
            "{:<4}%  {}  {:<25}  {:<12}  {}",
            fee_num,
            self.pool.address(None, None),
            self.pool.liquidity,
            self.pool.tick_current,
            rate,
        );
        Ok(())
    }
}
