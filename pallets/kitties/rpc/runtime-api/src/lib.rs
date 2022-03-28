#![cfg_attr(not(feature = "std"), no_std)]


pub use pallet_kitties::{ Gender };


sp_api::decl_runtime_apis! {
	pub trait KittiesRuntimeApi
	{
		fn get_kitty_runtime(kitty_id: sp_core::H256) -> ([u8; 16], Gender);
	}
}
