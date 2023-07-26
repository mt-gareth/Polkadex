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

//! This crate provides an RPC methods for OCEX pallet - balances state and onchain/offchain
//! recovery data.

use jsonrpsee::{
	core::{async_trait, Error as JsonRpseeError, RpcResult},
	proc_macros::rpc,
	types::error::{CallError, ErrorObject},
};
use orderbook_primitives::recovery::ObRecoveryState;
pub use pallet_ocex_runtime_api::PolkadexOcexRuntimeApi;
use parity_scale_codec::{Codec, Decode};
use polkadex_primitives::AssetId;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::traits::Block as BlockT;
use std::sync::Arc;

const RUNTIME_ERROR: i32 = 1;

#[rpc(client, server)]
pub trait PolkadexOcexRpcApi<BlockHash, AccountId, Hash> {
	#[method(name = "ob_getRecoverState")]
	fn get_ob_recover_state(&self, at: Option<BlockHash>) -> RpcResult<ObRecoveryState>;

	#[method(name = "ob_getBalance")]
	fn get_balance(
		&self,
		account_id: AccountId,
		of: AssetId,
		at: Option<BlockHash>,
	) -> RpcResult<String>;
}

/// A structure that represents the Polkadex Rewards RPC, which allows querying
/// rewards-related information through remote procedure calls.
///
/// # Type Parameters
///
/// * `Client`: The client API used to interact with the Substrate runtime.
/// * `Block`: The block type of the Substrate runtime.
pub struct PolkadexOcexRpc<Client, Block> {
	/// An `Arc` reference to the client API for accessing runtime functionality.
	client: Arc<Client>,

	/// A marker for the `Block` type parameter, used to ensure the struct
	/// is covariant with respect to the block type.
	_marker: std::marker::PhantomData<Block>,
}

impl<Client, Block> PolkadexOcexRpc<Client, Block> {
	pub fn new(client: Arc<Client>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

#[async_trait]
impl<Client, Block, AccountId, Hash>
	PolkadexOcexRpcApiServer<<Block as BlockT>::Hash, AccountId, Hash>
	for PolkadexOcexRpc<Client, Block>
where
	Block: BlockT,
	Client: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
	Client::Api: PolkadexOcexRuntimeApi<Block, AccountId, Hash>,
	AccountId: Codec,
	Hash: Codec,
{
	fn get_ob_recover_state(
		&self,
		at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<ObRecoveryState> {
		let api = self.client.runtime_api();
		let at = match at {
			Some(at) => at,
			None => self.client.info().best_hash,
		};
		// WARN: this is a hack on beating the boundry of runtime -> node
		// with decoding tuple of underlying data into solid std type
		Decode::decode(
			&mut api
				.get_ob_recover_state(at)
				.map_err(runtime_error_into_rpc_err)?
				.map_err(runtime_error_into_rpc_err)?
				.as_ref(),
		)
		.map_err(runtime_error_into_rpc_err)
	}

	fn get_balance(
		&self,
		account_id: AccountId,
		of: AssetId,
		at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<String> {
		let api = self.client.runtime_api();
		let at = match at {
			Some(at) => at,
			None => self.client.info().best_hash,
		};
		let runtime_api_result =
			api.get_balance(at, account_id, of).map_err(runtime_error_into_rpc_err)?;
		let json =
			serde_json::to_string(&runtime_api_result).map_err(runtime_error_into_rpc_err)?;
		Ok(json)
	}
}

/// Converts a runtime trap into an RPC error.
fn runtime_error_into_rpc_err(err: impl std::fmt::Debug) -> JsonRpseeError {
	CallError::Custom(ErrorObject::owned(RUNTIME_ERROR, "Runtime error", Some(format!("{err:?}"))))
		.into()
}
