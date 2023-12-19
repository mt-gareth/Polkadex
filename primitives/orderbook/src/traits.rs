use frame_support::dispatch::DispatchResult;
use rust_decimal::Decimal;
use crate::types::TradingPair;

pub trait LiquidityMiningCrowdSourcePallet<AccountId> {
    fn add_liquidity_success(
        market: TradingPair,
        pool: &AccountId,
        lp: &AccountId,
        shared_issued: Decimal,
        price: Decimal,
        total_inventory_in_quote: Decimal
    ) -> DispatchResult;

    fn remove_liquidity_success(
        market: TradingPair,
        pool: &AccountId,
        lp: &AccountId,
        base_free: Decimal,
        quote_free: Decimal,
    ) -> DispatchResult;

    fn remove_liquidity_failed(
        market: TradingPair,
        pool: &AccountId,
        lp: &AccountId,
        burn_frac: Decimal,
        base_free: Decimal,
        quote_free: Decimal,
        base_required: Decimal,
        quote_required: Decimal,
    ) -> DispatchResult;

    fn pool_force_close_success(
        market: TradingPair,
        pool: &AccountId,
        base_freed: Decimal,
        quote_freed: Decimal
    ) -> DispatchResult;

    fn stop_accepting_lmp_withdrawals(epoch: u16);
}