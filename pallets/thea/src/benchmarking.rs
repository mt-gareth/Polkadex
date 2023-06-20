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
use frame_benchmarking::benchmarks;
use frame_support::BoundedVec;
use frame_system::RawOrigin;
use parity_scale_codec::Decode;

// Check if last event generated by pallet is the one we're expecting
fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

const SIG: [u8; 48] = [
	149, 78, 11, 39, 209, 149, 209, 101, 74, 132, 154, 96, 46, 218, 114, 207, 95, 52, 40, 70, 44,
	13, 7, 236, 224, 87, 192, 58, 99, 125, 175, 25, 35, 186, 6, 53, 246, 152, 164, 191, 169, 212,
	133, 30, 143, 196, 55, 214,
];

const PK: [u8; 96] = [
	128, 68, 92, 111, 149, 140, 246, 244, 137, 50, 23, 217, 197, 153, 235, 255, 228, 58, 108, 191,
	41, 203, 237, 112, 203, 173, 118, 41, 92, 3, 165, 18, 200, 173, 125, 232, 182, 162, 9, 122, 13,
	77, 41, 222, 92, 53, 60, 0, 22, 227, 136, 163, 35, 121, 27, 34, 208, 233, 191, 74, 36, 223, 17,
	34, 79, 35, 164, 208, 138, 207, 171, 53, 254, 213, 17, 141, 35, 196, 81, 247, 20, 171, 33, 187,
	152, 79, 229, 3, 121, 17, 242, 252, 147, 209, 50, 186,
];

benchmarks! {
	update_network_pref {
		let b in 1 .. 255; // keep within u8 range
		let authority = <T as crate::Config>::TheaId::decode(&mut PK.as_ref()).unwrap();
		let network = b as u8;
		let signature = <T as crate::Config>::Signature::decode(&mut SIG.as_ref()).unwrap();
	}: _(RawOrigin::None, authority.clone(), network, signature)
	verify {
		assert_last_event::<T>(Event::NetworkUpdated { authority, network}.into());
	}

	incoming_message {
		let b in 0 .. 256; // keep withing u8 range
		let key = <T as crate::Config>::TheaId::decode(&mut PK.as_ref()).unwrap();
		let signature = <T as crate::Config>::Signature::decode(&mut SIG.as_ref()).unwrap();
		let message = Message {
			block_no: u64::MAX,
			nonce: 1,
			data: [255u8; 576].into(), //10 MB
			network: 0u8,
			is_key_change: false,
			validator_set_id: 0,
			validator_set_len: 1,
		};
		let mut set: BoundedVec<<T as crate::Config>::TheaId, <T as crate::Config>::MaxAuthorities> = BoundedVec::with_bounded_capacity(1);
		set.try_push(key).unwrap();
		<Authorities::<T>>::insert(0, 0, set);
		let bitmap = vec!(u128::MAX); // ALL bits are set :)
	}: _(RawOrigin::None, bitmap, message, signature)
	verify {
		assert!(<IncomingNonce::<T>>::get(0) == 1);
		assert!(<IncomingMessages::<T>>::iter().count() == 1);
	}

	send_thea_message {
		let b in 0 .. 256; // keep within u8 bounds
		let key = <T as crate::Config>::TheaId::decode(&mut PK.as_ref()).unwrap();
		let network = b as u8;
		let data = [b as u8; 1_048_576].to_vec(); // 10MB
		let mut set: BoundedVec<<T as crate::Config>::TheaId, <T as crate::Config>::MaxAuthorities> = BoundedVec::with_bounded_capacity(1);
		set.try_push(key).unwrap();
		<Authorities::<T>>::insert(network, 0, set);
	}: _(RawOrigin::Root, data, network)
	verify {
		assert!(<OutgoingNonce::<T>>::get(network) == 1);
		assert!(<OutgoingMessages::<T>>::iter().count() == 1);
	}

	update_incoming_nonce {
		let b in 1 .. u32::MAX;
		let network = 0;
		let nonce: u64 = b.into();
	}: _(RawOrigin::Root, nonce, network)
	verify {
		assert!(<IncomingNonce::<T>>::get(network) == nonce);
	}

	update_outgoing_nonce {
		let b in 1 .. u32::MAX;
		let network = 0;
		let nonce: u64 = b.into();
	}: _(RawOrigin::Root, nonce, network)
	verify {
		assert!(<OutgoingNonce::<T>>::get(network) == nonce);
	}
}

#[cfg(test)]
use frame_benchmarking::impl_benchmark_test_suite;

#[cfg(test)]
impl_benchmark_test_suite!(Thea, crate::mock::new_test_ext(), crate::mock::Test);