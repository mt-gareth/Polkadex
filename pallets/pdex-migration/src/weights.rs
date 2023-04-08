
//! Autogenerated weights for `pdex_migration`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-04-07, STEPS: `100`, REPEAT: 200, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `Ubuntu-2204-jammy-amd64-base`, CPU: `Intel(R) Core(TM) i7-7700 CPU @ 3.60GHz`
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// ./polkadex-node
// benchmark
// pallet
// --pallet
// pdex_migration
// --steps
// 100
// --repeat
// 200
// --extrinsic
// *
// --output
// migration_weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pdex_migration`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pdex_migration::WeightInfo for WeightInfo<T> {
	// Storage: PDEXMigration Operational (r:0 w:1)
	fn set_migration_operational_status() -> Weight {
		// Minimum execution time: 4_612 nanoseconds.
		Weight::from_ref_time(4_806_000)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: PDEXMigration Relayers (r:0 w:1)
	fn set_relayer_status() -> Weight {
		// Minimum execution time: 14_819 nanoseconds.
		Weight::from_ref_time(15_203_000)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: PDEXMigration Operational (r:1 w:0)
	// Storage: PDEXMigration EthTxns (r:1 w:1)
	// Storage: PDEXMigration Relayers (r:1 w:0)
	// Storage: PDEXMigration MintableTokens (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: PDEXMigration LockedTokenHolders (r:0 w:1)
	/// The range of component `b` is `[1, 254]`.
	fn mint(_b: u32, ) -> Weight {
		// Minimum execution time: 56_150 nanoseconds.
		Weight::from_ref_time(58_022_134)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: PDEXMigration Operational (r:1 w:0)
	// Storage: PDEXMigration LockedTokenHolders (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `b` is `[1, 254]`.
	fn unlock(_b: u32, ) -> Weight {
		// Minimum execution time: 35_728 nanoseconds.
		Weight::from_ref_time(36_771_389)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: PDEXMigration LockedTokenHolders (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: PDEXMigration MintableTokens (r:1 w:1)
	/// The range of component `b` is `[1, 254]`.
	fn remove_minted_tokens(b: u32, ) -> Weight {
		// Minimum execution time: 50_905 nanoseconds.
		Weight::from_ref_time(52_357_094)
			// Standard Error: 29
			.saturating_add(Weight::from_ref_time(30).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
}