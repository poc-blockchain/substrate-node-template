use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{
	generic::BlockId,
	traits::{Block as BlockT},
};
use std::sync::Arc;

use pallet_kitties_rpc_runtime_api:: { KittiesRuntimeApi, Gender };

#[rpc]
pub trait KittiesApi<BlockHash> {
	#[rpc(name = "kitty_get")]
	fn get_kitty(&self, at: Option<BlockHash>, kitty_id: sp_core::H256) -> Result<([u8; 16], Gender)>;
	#[rpc(name = "kitty_count")]
	fn count_kitty(&self, at: Option<BlockHash>) -> Result<u64>;
}

pub struct Kitties<C, P> {
	client: Arc<C>,
	_marker: std::marker::PhantomData<P>,
}

impl<C, P> Kitties<C, P> {
	/// Create new `TransactionPayment` with the given reference to the client.
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

/// Error type of this RPC api.
pub enum Error {
	/// The transaction was not decodable.
	DecodeError,
	/// The call to runtime failed.
	RuntimeError,
}

impl From<Error> for i64 {
	fn from(e: Error) -> i64 {
		match e {
			Error::RuntimeError => 1,
			Error::DecodeError => 2,
		}
	}
}

impl<C, Block> KittiesApi<<Block as BlockT>::Hash>
	for Kitties<C, Block>
where
	Block: BlockT,
	C: 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
	C::Api: KittiesRuntimeApi<Block>,
{
	fn get_kitty(
		&self,
		at: Option<<Block as BlockT>::Hash>,
		kitty_id: sp_core::H256
	) -> Result<([u8; 16], Gender)> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));
		
		// let kitty_id_str = sp_core::H256::zero();
		let result_api = api.get_kitty_runtime(&at, kitty_id);

		result_api.map_err(|e| RpcError {
			code: ErrorCode::ServerError(Error::RuntimeError.into()),
			message: "Unable to query dispatch info.".into(),
			data: Some(e.to_string().into()),
		})
	}

	fn count_kitty(
		&self,
		at: Option<<Block as BlockT>::Hash>,
	) -> Result<u64> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));
		
		let result_api = api.count_kitties_runtime(&at);

		result_api.map_err(|e| RpcError {
			code: ErrorCode::ServerError(Error::RuntimeError.into()),
			message: "Unable to query dispatch info.".into(),
			data: Some(e.to_string().into()),
		})
	}
	
} 