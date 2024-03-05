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
		// Minimum execution time: 153_620_000 picoseconds.
		Weight::from_parts(156_605_432, 0)
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
		// Minimum execution time: 1_222_364_000 picoseconds.
		Weight::from_parts(1_248_254_363, 0)
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
		// Minimum execution time: 15_310_000 picoseconds.
		Weight::from_parts(16_168_331, 0)
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
		// Minimum execution time: 15_440_000 picoseconds.
		Weight::from_parts(16_294_204, 0)
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
		// Minimum execution time: 25_210_000 picoseconds.
		Weight::from_parts(25_790_000, 0)
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
		// Minimum execution time: 23_710_000 picoseconds.
		Weight::from_parts(24_590_000, 0)
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
		// Minimum execution time: 75_710_000 picoseconds.
		Weight::from_parts(77_040_000, 0)
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
		// Minimum execution time: 167_181_000 picoseconds.
		Weight::from_parts(168_711_000, 0)
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
		// Minimum execution time: 313_741_000 picoseconds.
		Weight::from_parts(316_631_000, 0)
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
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(1249), added: 3724, mode: `MaxEncodedLen`)
	/// Storage: `Thea::IncomingMessages` (r:0 w:232)
	/// Proof: `Thea::IncomingMessages` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `x` is `[1, 1000]`.
	fn on_initialize(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `16662 + x * (18 ±0)`
		//  Estimated: `262740 + x * (152 ±0)`
		// Minimum execution time: 186_350_000 picoseconds.
		Weight::from_parts(15_946_699_258, 0)
			.saturating_add(Weight::from_parts(0, 262740))
			// Standard Error: 380_746
			.saturating_add(Weight::from_parts(7_282_636, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(200))
			.saturating_add(T::DbWeight::get().writes(295))
			.saturating_add(Weight::from_parts(0, 152).saturating_mul(x.into()))
	}
}
