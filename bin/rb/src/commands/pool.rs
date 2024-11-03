use clap::{Args, Subcommand};
use eyre::Result;

use alloy::primitives::{address, Address};
use lib::prelude::*;

use tokens::erc20::Erc20;
use tokens::erc20_constants;
use uniswap_v3_sdk::prelude::FeeAmount;
use uniswapv3pool::pool_calcs::fee_to_float;
use uniswapv3pool::pool_constants;
use uniswapv3pool::univ3contract::UniswapV3PoolContract;
use uniswapv3pool::univ3sdk::UniswapV3PoolSdk;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct PoolArgs {
    #[command(subcommand)]
    pub command: PoolCommands,
}

#[derive(Debug, Subcommand)]
pub enum PoolCommands {
    TickSpacing(TickSpacingArgs),
    CurrentTick(CurrentTickArgs),
    List(ListArgs),
    Dump(DumpArgs),
    Info(InfoArgs),
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct TickSpacingArgs {
    pub pool_address: Address,
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct CurrentTickArgs {
    pub pool_address: Address,
}

const ADDR_ZERO: &str = "0000000000000000000000000000000000000000";
const ADDRESS_ZERO: Address = address!("0000000000000000000000000000000000000000");

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct ListArgs {
    #[arg(
        required = false,
        help = "UniswapV3Factory address",
        default_value = ADDR_ZERO,
        env
    )]
    pub factory_address: Address,
    #[arg(
        required = false,
        help = "Token A address",
        default_value = ADDR_ZERO,
        env
    )]
    pub token_one_address: Address,
    #[arg(
        required = false,
        help = "Token B address",
        default_value = ADDR_ZERO,
        env
    )]
    pub token_two_address: Address,
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct DumpArgs {
    pub factory_address: Address,
    pub token_one_address: Address,
    pub token_two_address: Address,
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct InfoArgs {
    pub factory_address: Address,
    pub token_one_address: Address,
    pub token_two_address: Address,
}

pub async fn pool_tick_spacing(args: TickSpacingArgs, provider: RootProvider) -> Result<()> {
    let pool_contract = UniswapV3PoolContract::new(args.pool_address, provider).await?;
    let tick_spacing = pool_contract.tick_spacing().await?;

    println!(" Tick Spacing: {}", tick_spacing);
    Ok(())
}

pub async fn pool_current_tick(args: CurrentTickArgs, provider: RootProvider) -> Result<()> {
    let pool_contract = UniswapV3PoolContract::new(args.pool_address, provider).await?;
    let result = pool_contract.current_tick().await?;

    println!(" Current Tick: {}", result);
    Ok(())
}

pub async fn pool_list(args: ListArgs, provider: RootProvider) -> Result<()> {
    let id = provider.get_chain_id().await?;
    let ListArgs {
        mut factory_address,
        mut token_one_address,
        mut token_two_address,
    } = args;

    if factory_address == ADDRESS_ZERO
        || token_one_address == ADDRESS_ZERO
        || token_two_address == ADDRESS_ZERO
    {
        let def_factory;
        let def_token_one;
        let def_token_two;

        match get_defaults(id) {
            Ok(res) => {
                def_factory = res.0;
                def_token_one = res.1;
                def_token_two = res.2;
            }
            Err(error) => {
                panic!("Error {:?}", error);
            }
        }
        if factory_address == ADDRESS_ZERO {
            factory_address = def_factory;
            println!("Using default factory: {}", factory_address);
        }
        if token_one_address == ADDRESS_ZERO {
            token_one_address = def_token_one;
            println!("Using default token A: {}", token_one_address);
        }
        if token_two_address == ADDRESS_ZERO {
            token_two_address = def_token_two;
            println!("Using default token B: {}", token_two_address);
        }
    }
    let tok_one_contract = Erc20::new(token_one_address, provider.clone()).await?;
    let tok_one_symbol = tok_one_contract.symbol().await?;
    let tok_two_contract = Erc20::new(token_two_address, provider.clone()).await?;
    let tok_two_symbol = tok_two_contract.symbol().await?;

    println!("Liquidity Pools for:");
    println!(" Factory: {}", factory_address);
    println!(" Token A: {:<7}  {}", tok_one_symbol, token_one_address);
    println!(" Token B: {:<7}  {}", tok_two_symbol, token_two_address);

    let fees = &[
        FeeAmount::LOWEST,
        //FeeAmount::LOW_200,
        //FeeAmount::LOW_300,
        //FeeAmount::LOW_400,
        FeeAmount::LOW,
        FeeAmount::MEDIUM,
        FeeAmount::HIGH,
    ];

    println!("Fee    Pool Address                                Liquidity                  Current Tick  Rate");
    for fee in fees {
        match UniswapV3PoolSdk::from_pool_key(
            id,
            factory_address,
            token_one_address,
            token_two_address,
            *fee,
            provider.clone(),
            None,
        )
        .await
        {
            Ok(pool) => {
                pool.one_line_info().ok();
            }
            Err(_error) => {
                let fee_num = fee_to_float(*fee);

                // let fee_num: usize = *fee as usize;
                // let fee_num = fee_num as f32;
                // let fee_num = fee_num / 10000.0;
                println!("{:<4}%  No liquidity pool", fee_num);
            }
        };
    }

    Ok(())
}

pub async fn pool_tick_dump(args: DumpArgs, provider: RootProvider) -> Result<()> {
    let DumpArgs {
        factory_address,
        token_one_address,
        token_two_address,
    } = args;

    let pool = UniswapV3PoolSdk::from_pool_key(
        13371,
        factory_address,
        token_one_address,
        token_two_address,
        FeeAmount::MEDIUM,
        provider,
        None,
    )
    .await?;
    let dump = pool.dump().await?;

    println!("Pool dump: {}", dump);
    Ok(())
}

pub async fn pool_tick_info(args: InfoArgs, provider: RootProvider) -> Result<()> {
    let InfoArgs {
        factory_address,
        token_one_address,
        token_two_address,
    } = args;

    let pool = UniswapV3PoolSdk::from_pool_key(
        13371,
        factory_address,
        token_one_address,
        token_two_address,
        FeeAmount::MEDIUM,
        provider,
        None,
    )
    .await?;
    let info = pool.info().await?;

    println!("Pool info: {}", info);
    Ok(())
}

fn get_defaults(id: u64) -> Result<(Address, Address, Address)> {
    match id {
        1 => Ok((
            pool_constants::ETHEREUM_UNISWAP_FACTORY,
            erc20_constants::ETHEREUM_WETH,
            erc20_constants::ETHEREUM_IMX,
        )),
        13371 => Ok((
            pool_constants::IMMUTABLE_QUICKSWAP_FACTORY,
            erc20_constants::IMMUTABLE_WETH,
            erc20_constants::IMMUTABLE_WIMX,
        )),
        _ => {
            panic!("No default values for chain {}", id)
        }
    }
}
