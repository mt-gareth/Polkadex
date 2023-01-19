use parity_scale_codec::{Decode, Encode};
use polkadex_primitives::BoundedVec;
use xcm::latest::{Fungibility, MultiLocation};
use xcm::latest::MultiAsset;
use crate::AssetIdConverter;
use scale_info::TypeInfo;
use sp_runtime::traits::ConstU32;

#[derive(Encode, Decode, Clone, TypeInfo, PartialEq, Debug)]
pub struct ParachainDeposit {
    pub recipient: MultiLocation,
    pub asset_and_amount: MultiAsset,
    pub deposit_nonce: u32,
    pub transaction_hash: sp_core::H256,
    pub network_id: u8
}

impl ParachainDeposit {
    pub fn convert_multi_asset_to_asset_id_and_amount(&self) -> Option<(u128, u128)> {
        let MultiAsset { id, fun } = self.asset_and_amount.clone();
        match fun {
            Fungibility::Fungible(fun) => {
                if let Some(asset) =
                self.get_asset_id()
                {
                    Some((asset, fun))
                } else {
                    None
                }
            },
            _ => None,
        }
    }
}

impl AssetIdConverter for ParachainDeposit {
    fn get_asset_id(&self) -> Option<u128> {
        if let Ok(asset_identifier) = BoundedVec::<u8, ConstU32<100>>::try_from(Encode::encode(self)) {
            let identifier_length = asset_identifier.len();
            let mut derived_asset_id: Vec<u8> = vec![];
            derived_asset_id.push(self.network_id);
            derived_asset_id.push(identifier_length as u8);
            derived_asset_id.extend(&asset_identifier.to_vec());
            let derived_asset_id_hash = &sp_io::hashing::keccak_256(derived_asset_id.as_ref())[0..16];
            let mut temp = [0u8; 16];
            temp.copy_from_slice(derived_asset_id_hash);
            Some(u128::from_le_bytes(temp))
        } else {
            None
        }
    }

    fn to_asset_id(&self) -> Self {
        todo!()
    }
}