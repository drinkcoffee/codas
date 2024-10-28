use alloy::{
    primitives::{Address},
    providers::Provider,
    sol,
};

use eyre::Result;

use alloy::transports::Transport;

use crate::univ3contract::IUniswapV3Pool::IUniswapV3PoolInstance;



// Uniswap V3 Pool from: https://github.com/Uniswap/v3-core/tree/v1.0.0/contracts/interfaces
sol!{
    #[allow(missing_docs)]
    #[sol(rpc)]
    interface IUniswapV3Pool {
        /// @title Pool state that never changes
        /// @notice These parameters are fixed for a pool forever, i.e., the methods will always return the same values
        //interface IUniswapV3PoolImmutables {

        /// @notice The contract that deployed the pool, which must adhere to the IUniswapV3Factory interface
        /// @return The contract address
        function factory() external view returns (address);

        /// @notice The first of the two tokens of the pool, sorted by address
        /// @return The token contract address
        function token0() external view returns (address);

        /// @notice The second of the two tokens of the pool, sorted by address
        /// @return The token contract address
        function token1() external view returns (address);

        /// @notice The pool's fee in hundredths of a bip, i.e. 1e-6
        /// @return The fee
        function fee() external view returns (uint24);

        /// @notice The pool tick spacing
        /// @dev Ticks can only be used at multiples of this value, minimum of 1 and always positive
        /// e.g.: a tickSpacing of 3 means ticks can be initialized every 3rd tick, i.e., ..., -6, -3, 0, 3, 6, ...
        /// This value is an int24 to avoid casting even though it is always positive.
        /// @return The tick spacing
        function tickSpacing() external view returns (int24);

        /// @notice The maximum amount of position liquidity that can use any tick in the range
        /// @dev This parameter is enforced per tick to prevent liquidity from overflowing a uint128 at any point, and
        /// also prevents out-of-range liquidity from being used to prevent adding in-range liquidity to a pool
        /// @return The max amount of liquidity per tick
        function maxLiquidityPerTick() external view returns (uint128);


        /// @title Pool state that can change
        /// @notice These methods compose the pool's state, and can change with any frequency including multiple times
        /// per transaction
        //    interface IUniswapV3PoolState {

        /// @notice The 0th storage slot in the pool stores many values, and is exposed as a single method to save gas
        /// when accessed externally.
        /// @return sqrtPriceX96 The current price of the pool as a sqrt(token1/token0) Q64.96 value
        /// tick The current tick of the pool, i.e. according to the last tick transition that was run.
        /// This value may not always be equal to SqrtTickMath.getTickAtSqrtRatio(sqrtPriceX96) if the price is on a tick
        /// boundary.
        /// observationIndex The index of the last oracle observation that was written,
        /// observationCardinality The current maximum number of observations stored in the pool,
        /// observationCardinalityNext The next maximum number of observations, to be updated when the observation.
        /// feeProtocol The protocol fee for both tokens of the pool.
        /// Encoded as two 4 bit values, where the protocol fee of token1 is shifted 4 bits and the protocol fee of token0
        /// is the lower 4 bits. Used as the denominator of a fraction of the swap fee, e.g. 4 means 1/4th of the swap fee.
        /// unlocked Whether the pool is currently locked to reentrancy
        function slot0()
            external
            view
            returns (
                uint160 sqrtPriceX96,
                int24 tick,
                uint16 observationIndex,
                uint16 observationCardinality,
                uint16 observationCardinalityNext,
                uint8 feeProtocol,
                bool unlocked
            );

        /// @notice The fee growth as a Q128.128 fees of token0 collected per unit of liquidity for the entire life of the pool
        /// @dev This value can overflow the uint256
        function feeGrowthGlobal0X128() external view returns (uint256);

        /// @notice The fee growth as a Q128.128 fees of token1 collected per unit of liquidity for the entire life of the pool
        /// @dev This value can overflow the uint256
        function feeGrowthGlobal1X128() external view returns (uint256);

        /// @notice The amounts of token0 and token1 that are owed to the protocol
        /// @dev Protocol fees will never exceed uint128 max in either token
        function protocolFees() external view returns (uint128 token0, uint128 token1);

        /// @notice The currently in range liquidity available to the pool
        /// @dev This value has no relationship to the total liquidity across all ticks
        function liquidity() external view returns (uint128);

        /// @notice Look up information about a specific tick in the pool
        /// @param tick The tick to look up
        /// @return liquidityGross the total amount of position liquidity that uses the pool either as tick lower or
        /// tick upper,
        /// liquidityNet how much liquidity changes when the pool price crosses the tick,
        /// feeGrowthOutside0X128 the fee growth on the other side of the tick from the current tick in token0,
        /// feeGrowthOutside1X128 the fee growth on the other side of the tick from the current tick in token1,
        /// tickCumulativeOutside the cumulative tick value on the other side of the tick from the current tick
        /// secondsPerLiquidityOutsideX128 the seconds spent per liquidity on the other side of the tick from the current tick,
        /// secondsOutside the seconds spent on the other side of the tick from the current tick,
        /// initialized Set to true if the tick is initialized, i.e. liquidityGross is greater than 0, otherwise equal to false.
        /// Outside values can only be used if the tick is initialized, i.e. if liquidityGross is greater than 0.
        /// In addition, these values are only relative and must be used only in comparison to previous snapshots for
        /// a specific position.
        function ticks(int24 tick)
            external
            view
            returns (
                uint128 liquidityGross,
                int128 liquidityNet,
                uint256 feeGrowthOutside0X128,
                uint256 feeGrowthOutside1X128,
                int56 tickCumulativeOutside,
                uint160 secondsPerLiquidityOutsideX128,
                uint32 secondsOutside,
                bool initialized
            );

        /// @notice Returns 256 packed tick initialized boolean values. See TickBitmap for more information
        function tickBitmap(int16 wordPosition) external view returns (uint256);

        /// @notice Returns the information about a position by the position's key
        /// @param key The position's key is a hash of a preimage composed by the owner, tickLower and tickUpper
        /// @return _liquidity The amount of liquidity in the position,
        /// Returns feeGrowthInside0LastX128 fee growth of token0 inside the tick range as of the last mint/burn/poke,
        /// Returns feeGrowthInside1LastX128 fee growth of token1 inside the tick range as of the last mint/burn/poke,
        /// Returns tokensOwed0 the computed amount of token0 owed to the position as of the last mint/burn/poke,
        /// Returns tokensOwed1 the computed amount of token1 owed to the position as of the last mint/burn/poke
        function positions(bytes32 key)
            external
            view
            returns (
                uint128 _liquidity,
                uint256 feeGrowthInside0LastX128,
                uint256 feeGrowthInside1LastX128,
                uint128 tokensOwed0,
                uint128 tokensOwed1
            );

        /// @notice Returns data about a specific observation index
        /// @param index The element of the observations array to fetch
        /// @dev You most likely want to use #observe() instead of this method to get an observation as of some amount of time
        /// ago, rather than at a specific index in the array.
        /// @return blockTimestamp The timestamp of the observation,
        /// Returns tickCumulative the tick multiplied by seconds elapsed for the life of the pool as of the observation timestamp,
        /// Returns secondsPerLiquidityCumulativeX128 the seconds per in range liquidity for the life of the pool as of the observation timestamp,
        /// Returns initialized whether the observation has been initialized and the values are safe to use
        function observations(uint256 index)
            external
            view
            returns (
                uint32 blockTimestamp,
                int56 tickCumulative,
                uint160 secondsPerLiquidityCumulativeX128,
                bool initialized
            );
    }
}

pub struct UniswapV3PoolContract<T, P> {
    pub pool_contract: IUniswapV3PoolInstance<T, P>,
}

impl<T, P> UniswapV3PoolContract<T, P> where 
    T: Transport + Clone,
    P: Provider<T> + Clone,
{
//    pub async fn new<T, P>(
    pub async fn new(
        token_address: Address,
        provider: P,
    ) -> Result<Self>
    where
        T: Transport + Clone,
        P: Provider<T> + Clone,
    {
        let pool_contract = IUniswapV3Pool::new(token_address, provider);
        Ok(Self{pool_contract})
    }

    pub async fn address(&self) -> Result<&Address> {
        let addr = self.pool_contract.address();
        Ok(addr)
    }

    pub async fn tick_spacing(&self) -> Result<i64> {
        let spacing = self.pool_contract.tickSpacing().call().await?._0;
        let spacing = spacing.as_i64();
        Ok(spacing)
    }

    pub async fn current_tick(&self) -> Result<i64> {
        let tick = self.pool_contract.slot0().call().await?.tick;
        let tick = tick.as_i64();
        Ok(tick)
    }

}