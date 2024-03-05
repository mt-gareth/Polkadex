//! Autogenerated weights for `thea`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-03-05, STEPS: `100`, REPEAT: `200`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-172-31-9-163`, CPU: `AMD EPYC 7571`
//! WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: 1024

// Executed Command:
// ./polkadex-node
// benchmark
// pallet
// --pallet
// thea
// --steps
// 100
// --repeat
// 200
// --extrinsic
// *
// --output
// thea_weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `thea`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> crate::TheaWeightInfo for WeightInfo<T> {
	/// Storage: `Thea::AllowListTestingRelayers` (r:1 w:0)
	/// Proof: `Thea::AllowListTestingRelayers` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Thea::NetworkConfig` (r:1 w:0)
	/// Proof: `Thea::NetworkConfig` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Thea::IncomingNonce` (r:1 w:0)
	/// Proof: `Thea::IncomingNonce` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Thea::IncomingMessagesQueue` (r:1 w:1)
	/// Proof: `Thea::IncomingMessagesQueue` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(1249), added: 3724, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[0, 256]`.
	fn submit_incoming_message(_b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `338`
		//  Estimated: `4714`
		// Minimum execution time: 154_130_000 picoseconds.
		Weight::from_parts(157_359_416, 0)
			.saturating_add(Weight::from_parts(0, 4714))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Thea::OutgoingNonce` (r:1 w:1)
	/// Proof: `Thea::OutgoingNonce` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Thea::OutgoingMessages` (r:0 w:1)
	/// Proof: `Thea::OutgoingMessages` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[0, 256]`.
	fn send_thea_message(_b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `145`
		//  Estimated: `3610`
		// Minimum execution time: 1_073_951_000 picoseconds.
		Weight::from_parts(1_101_902_919, 0)
			.saturating_add(Weight::from_parts(0, 3610))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Thea::IncomingNonce` (r:0 w:1)
	/// Proof: `Thea::IncomingNonce` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[1, 4294967295]`.
	fn update_incoming_nonce(_b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 15_560_000 picoseconds.
		Weight::from_parts(16_439_026, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Thea::OutgoingNonce` (r:0 w:1)
	/// Proof: `Thea::OutgoingNonce` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[1, 4294967295]`.
	fn update_outgoing_nonce(_b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 15_510_000 picoseconds.
		Weight::from_parts(16_338_761, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Thea::ActiveNetworks` (r:1 w:1)
	/// Proof: `Thea::ActiveNetworks` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Thea::NetworkConfig` (r:0 w:1)
	/// Proof: `Thea::NetworkConfig` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn add_thea_network() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `145`
		//  Estimated: `1630`
		// Minimum execution time: 25_280_000 picoseconds.
		Weight::from_parts(25_900_000, 0)
			.saturating_add(Weight::from_parts(0, 1630))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Thea::ActiveNetworks` (r:1 w:1)
	/// Proof: `Thea::ActiveNetworks` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn remove_thea_network() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `166`
		//  Estimated: `1651`
		// Minimum execution time: 23_880_000 picoseconds.
		Weight::from_parts(24_501_000, 0)
			.saturating_add(Weight::from_parts(0, 1651))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Thea::OutgoingMessages` (r:1 w:0)
	/// Proof: `Thea::OutgoingMessages` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Thea::SignedOutgoingMessages` (r:1 w:1)
	/// Proof: `Thea::SignedOutgoingMessages` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Thea::Authorities` (r:1 w:0)
	/// Proof: `Thea::Authorities` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Thea::SignedOutgoingNonce` (r:0 w:1)
	/// Proof: `Thea::SignedOutgoingNonce` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn submit_signed_outgoing_messages() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `556`
		//  Estimated: `4021`
		// Minimum execution time: 74_961_000 picoseconds.
		Weight::from_parts(76_060_000, 0)
			.saturating_add(Weight::from_parts(0, 4021))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Thea::NetworkConfig` (r:1 w:0)
	/// Proof: `Thea::NetworkConfig` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(1249), added: 3724, mode: `MaxEncodedLen`)
	/// Storage: `Thea::IncomingMessagesQueue` (r:1 w:1)
	/// Proof: `Thea::IncomingMessagesQueue` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Thea::MisbehaviourReports` (r:0 w:1)
	/// Proof: `Thea::MisbehaviourReports` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn report_misbehaviour() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `491`
		//  Estimated: `4714`
		// Minimum execution time: 166_310_000 picoseconds.
		Weight::from_parts(168_560_000, 0)
			.saturating_add(Weight::from_parts(0, 4714))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `Thea::MisbehaviourReports` (r:1 w:1)
	/// Proof: `Thea::MisbehaviourReports` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:2 w:2)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(1249), added: 3724, mode: `MaxEncodedLen`)
	fn handle_misbehaviour() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `856`
		//  Estimated: `8438`
		// Minimum execution time: 315_790_000 picoseconds.
		Weight::from_parts(318_430_000, 0)
			.saturating_add(Weight::from_parts(0, 8438))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `Thea::ActiveNetworks` (r:1 w:0)
	/// Proof: `Thea::ActiveNetworks` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Thea::IncomingNonce` (r:232 w:232)
	/// Proof: `Thea::IncomingNonce` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Thea::IncomingMessagesQueue` (r:232 w:232)
	/// Proof: `Thea::IncomingMessagesQueue` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `TheaExecutor::Metadata` (r:1 w:0)
	/// Proof: `TheaExecutor::Metadata` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(222), added: 2697, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(1249), added: 3724, mode: `MaxEncodedLen`)
	/// Storage: `Thea::IncomingMessages` (r:0 w:232)
	/// Proof: `Thea::IncomingMessages` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `x` is `[1, 1000]`.
	fn on_initialize(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `16709 + x * (18 ±0)`
		//  Estimated: `262787 + x * (152 ±0)`
		// Minimum execution time: 195_330_000 picoseconds.
		Weight::from_parts(15_935_480_424, 0)
			.saturating_add(Weight::from_parts(0, 262787))
			// Standard Error: 405_669
			.saturating_add(Weight::from_parts(9_493_854, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(201))
			.saturating_add(T::DbWeight::get().writes(296))
			.saturating_add(Weight::from_parts(0, 152).saturating_mul(x.into()))
	}
}
