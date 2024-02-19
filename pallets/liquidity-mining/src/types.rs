use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use polkadex_primitives::AssetId;
use rust_decimal::Decimal;
use scale_info::TypeInfo;

#[derive(Encode, Decode, MaxEncodedLen, TypeInfo)]
pub struct MarketMakerConfig<AccountId> {
	pub(crate) pool_id: AccountId,
	pub(crate) commission: Decimal,
	pub(crate) exit_fee: Decimal,
	pub(crate) public_funds_allowed: bool,
	pub(crate) name: [u8; 10],
	pub(crate) share_id: AssetId,
	pub(crate) force_closed: bool,
}

pub type EpochNumber = u32;