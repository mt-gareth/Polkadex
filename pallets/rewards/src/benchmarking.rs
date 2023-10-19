// This file is part of Polkadex.
//
// Copyright (c) 2023 Polkadex oü.
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

#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::{pallet::Call, Pallet as pallet_rewards};
use frame_benchmarking::v1::{account, benchmarks, whitelisted_caller, BenchmarkError};
use frame_support::{traits::UnfilteredDispatchable, traits::EnsureOrigin};
use frame_system::RawOrigin;
use parity_scale_codec::Decode;
use polkadex_primitives::UNIT_BALANCE;
use sp_runtime::traits::SaturatedConversion;

// Check if last event generated by pallet is the one we're expecting
fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

fn get_parameters_for_reward_cycle() -> (u64, u64, u32, u32) {
	(20, 120, 25, 1)
}
benchmarks! {
	create_reward_cycle {
		let b in 0..4838400;
		let i in 1..100;
		let r in 0..10;

		let origin = T::GovernanceOrigin::try_successful_origin().unwrap();
		let start_block = b as u32;
		let end_block = start_block + 1;

		let initial_percentage = i as u32;
		let reward_id = r as u32;
		let call = Call::<T>::create_reward_cycle {
			start_block: start_block.saturated_into(), end_block: end_block.saturated_into(), initial_percentage, reward_id };
	}: {call.dispatch_bypass_filter(origin)?}
	verify {
		assert_last_event::<T>(Event::RewardCycleCreated {
			start_block: start_block.saturated_into(),
			end_block: end_block.saturated_into(),
			reward_id
		}.into());
	}

	initialize_claim_rewards
	{
		let (start_block, end_block, initial_percentage, reward_id) =
			get_parameters_for_reward_cycle();

		//insert reward info into storage
		let reward_info = RewardInfo { start_block: start_block.saturated_into(), end_block: end_block.saturated_into(), initial_percentage };
		<InitializeRewards<T>>::insert(reward_id, reward_info);
		let someone: [u8; 32] =
			[
			254, 243, 86, 10, 107, 201, 46, 29, 70, 6, 204, 171, 233, 231, 178, 8, 147, 180, 143,
			59, 167, 7, 203, 235, 194, 253, 133, 67, 99, 107, 26, 7,
		];
		let alice_account = T::AccountId::decode(&mut someone.as_ref()).unwrap();
		let pallet_id_account = pallet_rewards::<T>::get_pallet_account();

		//set balance for pallet account
		T::NativeCurrency::deposit_creating(
			&pallet_id_account,
			(10000000 * UNIT_BALANCE).saturated_into(),
		);

		//set existential balance for alice
		T::NativeCurrency::deposit_creating(
			&alice_account,
			(10000000 * UNIT_BALANCE).saturated_into(),
		);

		frame_system::Pallet::<T>::set_block_number((end_block+1).saturated_into());

		let call = Call::<T>::initialize_claim_rewards {
			reward_id };
	}: { call.dispatch_bypass_filter(RawOrigin::Signed(alice_account.clone()).into())? }
	verify {
		assert_last_event::<T>(Event::UserUnlockedReward {
			user: alice_account,
			reward_id
		}.into());
	}

	claim {
		let (start_block, end_block, initial_percentage, reward_id) =
			get_parameters_for_reward_cycle();

		//insert reward info into storage
		let reward_info = RewardInfo { start_block: start_block.saturated_into(), end_block: end_block.saturated_into(), initial_percentage };
		<InitializeRewards<T>>::insert(reward_id, reward_info);

		let alice_account = account::<T::AccountId>("alice", 1, 0);

		let pallet_id_account = pallet_rewards::<T>::get_pallet_account();

		//set balance for pallet account
		T::NativeCurrency::deposit_creating(
			&pallet_id_account,
			(10000000 * UNIT_BALANCE).saturated_into(),
		);

		//set existential balance for alice
		T::NativeCurrency::deposit_creating(
			&alice_account,
			(10000000 * UNIT_BALANCE).saturated_into(),
		);

		frame_system::Pallet::<T>::set_block_number((end_block+1).saturated_into());

		// insert reward info into Storage
		let reward_info = RewardInfoForAccount {
			total_reward_amount: 200000000000000_u128.saturated_into(),
			claim_amount: 0_u128.saturated_into(),
			is_initial_rewards_claimed: false,
			is_initialized: true,
			lock_id: REWARDS_LOCK_ID,
			last_block_rewards_claim: get_parameters_for_reward_cycle().0.saturated_into(),
			initial_rewards_claimable: 50000000000000_u128.saturated_into(),
			factor: 1500000000000_u128.saturated_into(),
		};
		<Distributor<T>>::insert(reward_id, alice_account.clone(), reward_info);

		let call = Call::<T>::claim {
			reward_id };

	}: { call.dispatch_bypass_filter(RawOrigin::Signed(alice_account.clone()).into())? }
	verify {
		assert_last_event::<T>(Event::UserClaimedReward {
			user: alice_account.clone(),
			reward_id,
			claimed: (200 * UNIT_BALANCE).saturated_into(),
		}.into());
	}
}

#[cfg(test)]
use frame_benchmarking::impl_benchmark_test_suite;

#[cfg(test)]
impl_benchmark_test_suite!(pallet_rewards, crate::mock::new_test_ext(), crate::mock::Test);
