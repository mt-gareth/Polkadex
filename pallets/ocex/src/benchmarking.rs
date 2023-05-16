// This file is part of Polkadex.

// Copyright (C) 2020-2022 Polkadex oü.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Benchmarking setup for pallet-ocex
#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::{fixtures::SNAPSHOT, Pallet as Ocex};
use frame_benchmarking::{account, benchmarks};
use frame_support::{dispatch::UnfilteredDispatchable, traits::EnsureOrigin, BoundedVec};
use frame_system::RawOrigin;
use orderbook_primitives::Fees;
use parity_scale_codec::Decode;
use polkadex_primitives::{
	ocex::TradingPairConfig, withdrawal::Withdrawal, ProxyLimit, UNIT_BALANCE,
};
use rust_decimal::{prelude::*, Decimal};
use sp_runtime::{
	traits::{BlockNumberProvider, One},
	BoundedBTreeSet,
};

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
		let b in 0 .. 50_000;
		let origin = T::EnclaveOrigin::successful_origin();
		let account = T::EnclaveOrigin::successful_origin();
		let main: T::AccountId = match unsafe { origin.clone().into().unwrap_unchecked() } {
			RawOrigin::Signed(account) => account.into(),
			_ => panic!("wrong RawOrigin returned")
		};
		let proxy: T::AccountId = match unsafe { account.into().unwrap_unchecked() } {
			RawOrigin::Signed(account) => account.into(),
			_ => panic!("wrong RawOrigin returned")
		};
		<ExchangeState<T>>::put(true);
		let call = Call::<T>::register_main_account { proxy: proxy.clone() };
	}: { call.dispatch_bypass_filter(origin)? }
	verify {
		assert_last_event::<T>(Event::MainAccountRegistered {
			main,
			proxy
		}.into());
	}

	add_proxy_account {
		let x in 0 .. 255; // should not overflow u8
		let origin = T::EnclaveOrigin::successful_origin();
		let main: T::AccountId = match unsafe { origin.clone().into().unwrap_unchecked() } {
			RawOrigin::Signed(account) => account.into(),
			_ => panic!("wrong RawOrigin returned")
		};
		let proxy = T::AccountId::decode(&mut &[x as u8; 32].to_vec()[..]).unwrap();
		<ExchangeState<T>>::put(true);
		Ocex::<T>::register_main_account(origin.clone(), main.clone())?;
		let call = Call::<T>::add_proxy_account { proxy: proxy.clone() };
	}: { call.dispatch_bypass_filter(origin)? }
	verify {
		assert_last_event::<T>(Event::MainAccountRegistered {
			main,
			proxy
		}.into());
	}

	close_trading_pair {
		let x in 1 .. 50_000;
		let origin = T::GovernanceOrigin::successful_origin();
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
		let origin = T::GovernanceOrigin::successful_origin();
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
		let origin = T::GovernanceOrigin::successful_origin();
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
		let origin = T::GovernanceOrigin::successful_origin();
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
		let governance = T::GovernanceOrigin::successful_origin();
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
		let governance = T::GovernanceOrigin::successful_origin();
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
		let governance = T::GovernanceOrigin::successful_origin();
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
		let snapshot = SnapshotSummary::decode(&mut SNAPSHOT.as_ref()).unwrap();
		let call = Call::<T>::submit_snapshot { summary: snapshot };
	}: { call.dispatch_bypass_filter(RawOrigin::None.into())? }
	verify {
		assert!(<Snapshots<T>>::contains_key(1));
	}

	collect_fees {
		let x in 0 .. 255; // should not overflow u8
		let origin = T::GovernanceOrigin::successful_origin();
		let beneficiary = T::AccountId::decode(&mut &[x as u8; 32][..]).unwrap();
		let fees: Fees = Fees { asset: AssetId::Polkadex, amount: Decimal::new(100, 1) };
		<ExchangeState<T>>::put(true);
		let snapshot = SnapshotSummary::decode(&mut SNAPSHOT.as_ref()).unwrap();
		<Snapshots<T>>::insert(x as u64, snapshot);
		let call = Call::<T>::collect_fees { snapshot_id: x as u64, beneficiary: beneficiary.clone() };
	}: { call.dispatch_bypass_filter(origin)? }
	verify {
		assert_last_event::<T>(Event::FeesClaims{ beneficiary, snapshot_id: x as u64}.into());
	}

	set_exchange_state {
		let x in 0 .. 100_000;
		let state = x % 2 == 0;
		let origin = T::GovernanceOrigin::successful_origin();
		<ExchangeState<T>>::put(state);
		let call = Call::<T>::set_exchange_state { state: !state };
	}: { call.dispatch_bypass_filter(origin)? }
	verify {
		assert_eq!(<ExchangeState<T>>::get(), !state);
	}

	set_balances {
		let x in 0 .. 255; // should not overflow up
		let origin = T::GovernanceOrigin::successful_origin();
		let main_account = T::AccountId::decode(&mut &[x as u8; 32][..]).unwrap();
		let asset_id = AssetId::Asset(x as u128);
		let hb = polkadex_primitives::ingress::HandleBalance {
			main_account,
			asset_id,
			free: (x * 100) as u128,
			reserve: (x * 10) as u128
		};
		let mut change_in_balances: BoundedVec<
			polkadex_primitives::ingress::HandleBalance<T::AccountId>,
			polkadex_primitives::ingress::HandleBalanceLimit,
		> = BoundedVec::default();
		change_in_balances.try_push(hb).unwrap();
		let call = Call::<T>::set_balances { change_in_balances };
	}: { call.dispatch_bypass_filter(origin)? }
	verify {
		let current_blk = frame_system::Pallet::<T>::current_block_number();
		assert_eq!(<IngressMessages<T>>::get(current_blk).len(), 1);
	}

	claim_withdraw {
		let x in 1 .. 255; // should not overflow u8
		let governance = T::GovernanceOrigin::successful_origin();
		let origin = T::EnclaveOrigin::successful_origin();
		let main = T::AccountId::decode(&mut &[x as u8; 32][..]).unwrap();
		let asset = AssetId::Asset(x.into());
		let amount = BalanceOf::<T>::decode(&mut &(x as u128).to_le_bytes()[..]).unwrap();
		let mut vec_withdrawals = Vec::with_capacity(1);
		let fees = Decimal::new(100, 5);
		vec_withdrawals.push(Withdrawal {
			stid: 0,
			worker_nonce:0,
			amount: Decimal::new(x.into(), 0),
			stid:0,
			worker_nonce:0,
			asset,
			main_account: main.clone(),
			fees,
		});
		let mut wm = sp_std::collections::btree_map::BTreeMap::new();
		wm.insert(main.clone(), vec_withdrawals.clone()).unwrap();
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
	}: { call.dispatch_bypass_filter(origin)? }
	verify {
		assert_last_event::<T>(Event::WithdrawalClaimed {
			main,
			withdrawals: vec_withdrawals,
		}.into());
	}

	allowlist_token {
		let x in 0 .. 65_000;
		let origin = T::GovernanceOrigin::successful_origin();
		let asset_id = AssetId::Asset(x.into());
		<ExchangeState<T>>::put(true);
		let call = Call::<T>::allowlist_token { token: asset_id };
	}: { call.dispatch_bypass_filter(origin)? }
	verify {
		assert_last_event::<T>(Event::TokenAllowlisted(asset_id).into());
	}

	remove_allowlisted_token {
		let x in 0 .. 65_000;
		let origin = T::GovernanceOrigin::successful_origin();
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

	change_pending_withdrawal_limit {
		let origin = T::GovernanceOrigin::successful_origin();
		let call = Call::<T>::change_pending_withdrawal_limit { new_pending_withdrawals_limit: u64::MAX };
	}: { call.dispatch_bypass_filter(origin)? }

	change_snapshot_interval_block {
		let origin = T::GovernanceOrigin::successful_origin();
		let new_snapshot_interval_block = T::BlockNumber::decode(&mut 123u64.to_le_bytes().as_ref()).unwrap();
		let call = Call::<T>::change_snapshot_interval_block{ new_snapshot_interval_block };
	}: { call.dispatch_bypass_filter(origin)? }
}

#[cfg(test)]
use frame_benchmarking::impl_benchmark_test_suite;

#[cfg(test)]
impl_benchmark_test_suite!(Ocex, crate::mock::new_test_ext(), crate::mock::Test);
