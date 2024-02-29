// This file is part of Polkadex.
//
// Copyright (c) 2022-2023 Polkadex oü.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Benchmarking setup for pallet-ocex
#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::Pallet as Ocex;
use frame_benchmarking::{
    v1::{account, benchmarks},
    whitelisted_caller,
};
use frame_support::traits::{EnsureOrigin, UnfilteredDispatchable};
use frame_system::RawOrigin;
use orderbook_primitives::Fees;
use parity_scale_codec::Decode;
use polkadex_primitives::{
    ProxyLimit, UNIT_BALANCE, withdrawal::Withdrawal,
};
use rust_decimal::{Decimal, prelude::*};
use sp_runtime::{BoundedBTreeSet, traits::One};

// Check if last event generated by pallet is the one we're expecting
fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

fn convert_to_balance<T: Config>(dec: Decimal) -> BalanceOf<T> {
	BalanceOf::<T>::decode(
		&mut &dec.saturating_mul(UNIT_BALANCE.into()).to_u128().unwrap().to_le_bytes()[..],
	)
	.unwrap()
}

fn tpc(base_asset: AssetId, quote_asset: AssetId) -> TradingPairConfig {
	TradingPairConfig {
		base_asset,
		quote_asset,
		min_price: Decimal::from_f32(0.0001).unwrap(),
		max_price: Decimal::from_f32(100000.0).unwrap(),
		price_tick_size: Decimal::from_f32(0.000001).unwrap(),
		min_qty: Decimal::from_f64(0.001).unwrap(),
		max_qty: Decimal::from_f32(10000.0).unwrap(),
		qty_step_size: Decimal::from_f64(0.001).unwrap(),
		operational_status: true,
		base_asset_precision: 1,
		quote_asset_precision: 1,
	}
}

benchmarks! {
	register_main_account {
		let b in 0 .. 255;
		let main: T::AccountId = whitelisted_caller();
		let proxy = T::AccountId::decode(&mut &[b as u8; 32].to_vec()[..]).unwrap();
		<ExchangeState<T>>::put(true);
	}: _(RawOrigin::Signed(main.clone()), proxy.clone())
	verify {
		assert_last_event::<T>(Event::MainAccountRegistered {
			main,
			proxy
		}.into());
	}

	add_proxy_account {
		let x in 0 .. 255; // should not overflow u8
		let main: T::AccountId = whitelisted_caller();
		let proxy = T::AccountId::decode(&mut &[x as u8; 32].to_vec()[..]).unwrap();
		<ExchangeState<T>>::put(true);
		Ocex::<T>::register_main_account(RawOrigin::Signed(main.clone()).into(), main.clone())?;
	}: _(RawOrigin::Signed(main.clone()), proxy.clone())
	verify {
		assert_last_event::<T>(Event::NewProxyAdded {
			main,
			proxy
		}.into());
	}

	close_trading_pair {
		let x in 1 .. 50_000;
		let origin = T::GovernanceOrigin::try_successful_origin().unwrap();
		let base = AssetId::Asset(x.into());
		let quote = AssetId::Asset((x + 1).into());
		let config = tpc(base, quote);
		<TradingPairs<T>>::insert(base, quote, config);
		let pair = <TradingPairs<T>>::get(base, quote).unwrap();
		let expected_pair = TradingPairConfig {
			operational_status: false,
			..pair
		};
		<ExchangeState<T>>::put(true);
		let call = Call::<T>::close_trading_pair { base, quote };
	}: { call.dispatch_bypass_filter(origin)? }
	verify {
		assert_last_event::<T>(Event::ShutdownTradingPair {
			pair: expected_pair
		}.into());
	}

	open_trading_pair {
		let x in 0 .. 100_000;
		let origin = T::GovernanceOrigin::try_successful_origin().unwrap();
		let base = AssetId::Asset(x.into());
		let quote = AssetId::Asset((x + 1).into());
		let config = tpc(base, quote);
		<TradingPairs<T>>::insert(base, quote, config.clone());
		<ExchangeState<T>>::put(true);
		let call = Call::<T>::open_trading_pair { base, quote };
	}: { call.dispatch_bypass_filter(origin)? }
	verify {
		assert_last_event::<T>(Event::OpenTradingPair {
			pair: config,
		}.into());
	}

	register_trading_pair {
		let x in 0 .. 100_000;
		let origin = T::GovernanceOrigin::try_successful_origin().unwrap();
		let base = AssetId::Asset(x.into());
		let quote = AssetId::Asset((x + 1).into());
		let TradingPairConfig{
			base_asset,
			quote_asset,
			min_price,
			max_price,
			min_qty,
			max_qty,
			operational_status,
			price_tick_size,
			qty_step_size,
			base_asset_precision,
			quote_asset_precision,
			} = tpc(base, quote);
		<ExchangeState<T>>::put(true);
		let call = Call::<T>::register_trading_pair {
			base,
			quote,
			min_order_price: convert_to_balance::<T>(min_price),
			max_order_price: convert_to_balance::<T>(max_price),
			min_order_qty: convert_to_balance::<T>(min_qty),
			max_order_qty: convert_to_balance::<T>(max_qty),
			price_tick_size: convert_to_balance::<T>(price_tick_size),
			qty_step_size: convert_to_balance::<T>(qty_step_size)
		};
	}: { call.dispatch_bypass_filter(origin)? }
	verify {
		assert_last_event::<T>(Event::TradingPairRegistered {
			base,
			quote
		}.into());
	}

	update_trading_pair {
		let x in 0 .. 100_000;
		let origin = T::GovernanceOrigin::try_successful_origin().unwrap();
		let base = AssetId::Asset(x.into());
		let quote = AssetId::Asset((x + 1).into());
		let mut tp = tpc(base, quote);
		let TradingPairConfig{
			base_asset,
			quote_asset,
			min_price,
			max_price,
			min_qty,
			max_qty,
			operational_status,
			price_tick_size,
			qty_step_size,
			base_asset_precision,
			quote_asset_precision,
			} = tp.clone();
		let governance = T::GovernanceOrigin::try_successful_origin().unwrap();
		Ocex::<T>::set_exchange_state(governance.clone(), true)?;
		tp.operational_status = false;
		<TradingPairs<T>>::insert(base_asset, quote_asset, tp);
		let call = Call::<T>::update_trading_pair {
			base,
			quote,
			min_order_price: convert_to_balance::<T>(min_price),
			max_order_price: convert_to_balance::<T>(max_price),
			min_order_qty: convert_to_balance::<T>(min_qty),
			max_order_qty: convert_to_balance::<T>(max_qty),
			price_tick_size: convert_to_balance::<T>(price_tick_size),
			qty_step_size: convert_to_balance::<T>(qty_step_size)
		};
	}: { call.dispatch_bypass_filter(origin)? }
	verify {
		assert_last_event::<T>(Event::TradingPairUpdated {
			base,
			quote
		}.into());
	}

	deposit {
		let x in 1 .. 255; // should not overflow u8
		let user = account::<T::AccountId>("user", x, 0);
		let asset = AssetId::Asset(x.into());
		let amount  = BalanceOf::<T>::decode(&mut &(x as u128).saturating_mul(10u128).to_le_bytes()[..]).unwrap();
		let governance = T::GovernanceOrigin::try_successful_origin().unwrap();
		Ocex::<T>::set_exchange_state(governance.clone(), true)?;
		Ocex::<T>::allowlist_token(governance.clone(), asset.clone())?;
		use frame_support::traits::fungibles::Create;
		T::OtherAssets::create(
			x as u128,
			Ocex::<T>::get_pallet_account(),
			true,
			BalanceOf::<T>::one().unique_saturated_into())?;
		T::OtherAssets::mint_into(
			x as u128,
			&user.clone(),
			BalanceOf::<T>::decode(&mut &(u128::MAX).to_le_bytes()[..]).unwrap()
		)?;
		let proxy = account::<T::AccountId>("proxy", x, 0);
		Ocex::<T>::register_main_account(RawOrigin::Signed(user.clone()).into(), proxy)?;
		let call = Call::<T>::deposit { asset, amount };
	}: { call.dispatch_bypass_filter(RawOrigin::Signed(user.clone()).into())? }
	verify {
		assert_last_event::<T>(Event::DepositSuccessful {
			user,
			asset,
			amount
		}.into());
	}

	remove_proxy_account {
		let x in 1 .. 255; // should not overflow u8
		let main = account::<T::AccountId>("main", 0, 0);
		let proxy = T::AccountId::decode(&mut &[x as u8 ; 32].to_vec()[..]).unwrap();
		let governance = T::GovernanceOrigin::try_successful_origin().unwrap();
		Ocex::<T>::set_exchange_state(governance.clone(), true)?;
		let signed = RawOrigin::Signed(main.clone());
		Ocex::<T>::register_main_account(signed.clone().into(), proxy.clone())?;
		// worst case scenario
		for i in 2 .. ProxyLimit::get() {
			let new_proxy = account::<T::AccountId>("proxy", i, 0);
			Ocex::<T>::add_proxy_account(signed.clone().into(), new_proxy)?;
		}
		let call = Call::<T>::remove_proxy_account { proxy: proxy.clone() };
	}: { call.dispatch_bypass_filter(RawOrigin::Signed(main.clone()).into())? }
	verify {
		assert_last_event::<T>(Event::ProxyRemoved {
			main,
			proxy
		}.into());
	}

	submit_snapshot {
		<ExchangeState<T>>::put(true);
		let snapshot = get_dummy_snapshot::<T>();
		let call = Call::<T>::submit_snapshot { summary: snapshot, signatures: Vec::new() };
	}: { call.dispatch_bypass_filter(RawOrigin::None.into())? }
	verify {
		assert!(<Snapshots<T>>::contains_key(1));
	}

	collect_fees {
		let x in 0 .. 255; // should not overflow u8
		let origin = T::GovernanceOrigin::try_successful_origin().unwrap();
		let beneficiary = T::AccountId::decode(&mut &[x as u8; 32][..]).unwrap();
		let fees: Fees = Fees { asset: AssetId::Polkadex, amount: Decimal::new(100, 1) };
		<ExchangeState<T>>::put(true);
		let snapshot =  get_dummy_snapshot::<T>();
		<Snapshots<T>>::insert(x as u64, snapshot);
		let call = Call::<T>::collect_fees { snapshot_id: x as u64, beneficiary: beneficiary.clone() };
	}: { call.dispatch_bypass_filter(origin)? }
	verify {
		assert_last_event::<T>(Event::FeesClaims{ beneficiary, snapshot_id: x as u64}.into());
	}

	set_exchange_state {
		let x in 0 .. 100_000;
		let state = x % 2 == 0;
		let origin = T::GovernanceOrigin::try_successful_origin().unwrap();
		<ExchangeState<T>>::put(state);
		let call = Call::<T>::set_exchange_state { state: !state };
	}: { call.dispatch_bypass_filter(origin)? }
	verify {
		assert_eq!(<ExchangeState<T>>::get(), !state);
	}

	claim_withdraw {
		let x in 1 .. 255; // should not overflow u8
		let governance = T::GovernanceOrigin::try_successful_origin().unwrap();
		let origin = T::EnclaveOrigin::try_successful_origin().unwrap();
		let main = T::AccountId::decode(&mut &[x as u8; 32][..]).unwrap();
		let asset = AssetId::Asset(x.into());
		let amount = BalanceOf::<T>::decode(&mut &(x as u128).to_le_bytes()[..]).unwrap();
		let mut vec_withdrawals = Vec::with_capacity(1);
		let fees = Decimal::new(100, 5);
		vec_withdrawals.push(Withdrawal {
			amount: Decimal::new(x.into(), 0),
			stid:0,
			asset,
			main_account: main.clone(),
			fees,
		});
		let mut wm = sp_std::collections::btree_map::BTreeMap::new();
		wm.insert(main.clone(), vec_withdrawals.clone());
		<Withdrawals<T>>::insert(x as u64, wm);
		Ocex::<T>::set_exchange_state(governance.clone(), true)?;
		Ocex::<T>::allowlist_token(governance.clone(), asset.clone())?;
		use frame_support::traits::fungibles::Create;
		T::OtherAssets::create(
			x as u128,
			Ocex::<T>::get_pallet_account(),
			true,
			BalanceOf::<T>::one().unique_saturated_into())?;
		T::OtherAssets::mint_into(
			x as u128,
			&Ocex::<T>::get_pallet_account(),
			BalanceOf::<T>::decode(&mut &(u128::MAX).to_le_bytes()[..]).unwrap()
		)?;
		let call = Call::<T>::claim_withdraw { snapshot_id: x as u64, account: main.clone() };
	}: _(RawOrigin::Signed(main.clone()), x as u64, main.clone())
	verify {
		assert_last_event::<T>(Event::WithdrawalClaimed {
			main,
			withdrawals: vec_withdrawals,
		}.into());
	}

	allowlist_token {
		let x in 0 .. 65_000;
		let origin = T::GovernanceOrigin::try_successful_origin().unwrap();
		let asset_id = AssetId::Asset(x.into());
		<ExchangeState<T>>::put(true);
		let call = Call::<T>::allowlist_token { token: asset_id };
	}: { call.dispatch_bypass_filter(origin)? }
	verify {
		assert_last_event::<T>(Event::TokenAllowlisted(asset_id).into());
	}

	remove_allowlisted_token {
		let x in 0 .. 65_000;
		let origin = T::GovernanceOrigin::try_successful_origin().unwrap();
		let asset_id = AssetId::Asset(x.into());
		let mut at: BoundedBTreeSet<AssetId, AllowlistedTokenLimit> = BoundedBTreeSet::new();
		at.try_insert(asset_id).unwrap();
		<AllowlistedToken<T>>::put(at);
		<ExchangeState<T>>::put(true);
		let call = Call::<T>::remove_allowlisted_token { token: asset_id };
	}: { call.dispatch_bypass_filter(origin)? }
	verify {
		assert_last_event::<T>(Event::AllowlistedTokenRemoved(asset_id).into());
	}

	set_snapshot {
		let call = Call::<T>::set_snapshot{ new_snapshot_id: u64::MAX };
	}: { call.dispatch_bypass_filter(RawOrigin::Root.into())? }

	whitelist_orderbook_operator {
		let origin = T::GovernanceOrigin::try_successful_origin().unwrap();
		let operator_public_key = sp_core::ecdsa::Public([u8::MAX; 33]);
		let call = Call::<T>::whitelist_orderbook_operator { operator_public_key };
	}: { call.dispatch_bypass_filter(origin)? }
	verify {
		assert!(<OrderbookOperatorPublicKey<T>>::get().unwrap() == operator_public_key);
	}
}

fn get_dummy_snapshot<T: Config>() -> SnapshotSummary<T::AccountId> {
	let mut withdrawals = Vec::new();
	for _ in 0..20 {
		withdrawals.push(Withdrawal {
			main_account: T::AccountId::decode(&mut &[0u8; 32][..]).unwrap(),
			amount: Decimal::one(),
			asset: AssetId::Polkadex,
			fees: Decimal::one(),
			stid: 1,
		});
	}
	SnapshotSummary {
		validator_set_id: 10,
		snapshot_id: 1,
		state_hash: Default::default(),
		state_change_id: 10,
		last_processed_blk: 11,
		withdrawals,
		egress_messages: Vec::new(),
		trader_metrics: Default::default(),
	}
}

#[cfg(test)]
use frame_benchmarking::impl_benchmark_test_suite;
use orderbook_primitives::ocex::TradingPairConfig;

#[cfg(test)]
impl_benchmark_test_suite!(Ocex, crate::mock::new_test_ext(), crate::mock::Test);
