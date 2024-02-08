use crate::{
	pallet::{Config, ExpectedLMPConfig, LMPConfig, Pallet},
	FinalizeLMPScore, LMPEpoch,
};
use frame_system::pallet_prelude::BlockNumberFor;
use orderbook_primitives::traits::LiquidityMiningCrowdSourcePallet;
use sp_runtime::SaturatedConversion;

impl<T: Config> Pallet<T> {
	pub(crate) fn should_start_new_epoch(n: BlockNumberFor<T>) -> bool {
		n.saturated_into::<u32>() % 201600u32 == 0 // 28 days in blocks
	}

	/// Starts new liquidity mining epoch
	pub(crate) fn start_new_epoch() {
		let mut current_epoch: u16 = <LMPEpoch<T>>::get();
		if <FinalizeLMPScore<T>>::get().is_none() {
			<FinalizeLMPScore<T>>::put(current_epoch);
		}
		current_epoch = current_epoch.saturating_add(1);
		<LMPEpoch<T>>::put(current_epoch);
		let config = <ExpectedLMPConfig<T>>::get();
		<LMPConfig<T>>::insert(current_epoch, config);
		// Notify Liquidity Crowd sourcing pallet about new epoch
		T::CrowdSourceLiqudityMining::new_epoch(current_epoch);
	}

	pub(crate) fn should_stop_accepting_lmp_withdrawals(n: BlockNumberFor<T>) -> bool {
		// Triggers 7200 blocks ( or approx 1 day before epoch change)
		n.saturated_into::<u32>().saturating_add(7200) % 201600u32 == 0
	}

	pub(crate) fn stop_accepting_lmp_withdrawals() {
		let current_epoch: u16 = <LMPEpoch<T>>::get();
		T::CrowdSourceLiqudityMining::stop_accepting_lmp_withdrawals(current_epoch)
	}
}