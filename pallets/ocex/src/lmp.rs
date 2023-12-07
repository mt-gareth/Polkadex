use crate::{storage::OffchainState, Config, Pallet};
use orderbook_primitives::types::{OrderSide, Trade, TradingPair};
use parity_scale_codec::{Decode, Encode};
use polkadex_primitives::{ocex::TradingPairConfig, AccountId};
use rust_decimal::Decimal;

pub fn update_trade_volume_by_main_account(
	state: &mut OffchainState,
	epoch: u32,
	market: &TradingPairConfig,
	volume: Decimal,
	main: &AccountId,
) -> Result<(), &'static str> {
	let trading_pair = TradingPair::from(market.quote_asset, market.base_asset);
	let key = (epoch, trading_pair, "trading_volume", main).encode();
	match state.get(&key)? {
		None => state.insert(key, volume.encode()),
		Some(encoded_volume) => {
			let recorded_volume = Decimal::decode(&mut &encoded_volume[..])?;
			state.insert(key, recorded_volume.saturating_add(volume).encode());
		},
	}
	Ok(())
}

pub fn update_maker_volume_by_main_account(
	state: &mut OffchainState,
	epoch: u32,
	market: &TradingPairConfig,
	volume: Decimal,
	main: &AccountId,
) -> Result<(), &'static str> {
	let trading_pair = TradingPair::from(market.quote_asset, market.base_asset);
	let key = (epoch, trading_pair, "maker_volume", main).encode();
	match state.get(&key)? {
		None => state.insert(key, volume.encode()),
		Some(encoded_volume) => {
			let recorded_volume = Decimal::decode(&mut &encoded_volume[..])?;
			state.insert(key, recorded_volume.saturating_add(volume).encode());
		},
	}
	Ok(())
}

pub fn store_fees_paid_by_main_account_in_quote(
	state: &mut OffchainState,
	epoch: u32,
	market: &TradingPairConfig,
	fees_in_quote_terms: Decimal,
	main: &AccountId,
) -> Result<(), &'static str> {
	let trading_pair = TradingPair::from(market.quote_asset, market.base_asset);
	let key = (epoch, trading_pair, "fees_paid", main).encode();
	match state.get(&key)? {
		None => state.insert(key, fees_in_quote_terms.encode()),
		Some(encoded_fees_paid) => {
			let recorded_fees_paid = Decimal::decode(&mut &encoded_fees_paid[..])?;
			state.insert(key, recorded_fees_paid.saturating_add(fees_in_quote_terms).encode());
		},
	}
	Ok(())
}

impl<T: Config> Pallet<T> {
	pub fn update_lmp_storage_from_trade(
		state: &mut OffchainState,
		trade: &Trade,
		config: TradingPairConfig,
		maker_fees: Decimal,
		taker_fees: Decimal,
	) -> Result<(), &'static str> {
		let epoch: u32 = <LMPEpoch<T>>::get();

		// Store trade.price * trade.volume as maker volume for this epoch
		let volume = trade.price.saturating_mul(trade.amount);
		update_trade_volume_by_main_account(
			state,
			epoch,
			&config,
			volume,
			&trade.maker.main_account,
		)?;
		update_trade_volume_by_main_account(
			state,
			epoch,
			&config,
			volume,
			&trade.taker.main_account,
		)?;
		update_maker_volume_by_main_account(
			state,
			epoch,
			&config,
			volume,
			&trade.maker.main_account,
		)?;

		// Store maker_fees and taker_fees for the corresponding main account for this epoch
		match trade.maker.side {
			OrderSide::Ask => {
				let fees = maker_fees; // Maker fees is in quote because they put ask order.
				store_fees_paid_by_main_account_in_quote(
					state,
					epoch,
					&config,
					fees,
					&trade.maker.main_account,
				)?;

				// Convert taker fees to quote terms based on trade price
				let fees = taker_fees.saturating_mul(trade.price);
				store_fees_paid_by_main_account_in_quote(
					state,
					epoch,
					&config,
					fees,
					&trade.taker.main_account,
				)?;
			},
			OrderSide::Bid => {
				// Convert maker fees to quote terms based on trade price
				let fees = maker_fees.saturating_mul(trade.price);
				store_fees_paid_by_main_account_in_quote(
					state,
					epoch,
					&config,
					fees,
					&trade.maker.main_account,
				)?;

				// Taker fees is in quote because they put bid order.
				let fees = taker_fees.saturating_mul(trade.price);
				store_fees_paid_by_main_account_in_quote(
					state,
					epoch,
					&config,
					fees,
					&trade.taker.main_account,
				)?;
			},
		}
		Ok(())
	}
}
