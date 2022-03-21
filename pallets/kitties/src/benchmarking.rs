//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as KittiesPallet;
use frame_benchmarking::{benchmarks, whitelisted_caller, account};
use frame_system::RawOrigin;

benchmarks! {
	create_kitty {
		let s in 0 .. 10000;
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))
	verify {
        let s_u64: u64 = s.into();
		assert_eq!(KittyCnt::<T>::get(), 1);
	}

	set_price {
		let s in 0 .. 10000;
		let owner: T::AccountId = account("owner", 3u32, 3u32);
		let kitty_id = KittiesPallet::<T>::mint(&owner, Some([1u8; 16]), Some(pallet::Gender::Female)).unwrap();
	}: _(RawOrigin::Signed(owner), kitty_id, Some(s.into()))
	verify {
		// Get price to compare
		let kitty = KittiesPallet::<T>::kitties(&kitty_id).unwrap();
		assert_eq!(kitty.price, Some(s.into()));
	}

	transfer {
		let s in 0 .. 10000;
		let receiver: T::AccountId = whitelisted_caller();
		let owner: T::AccountId = account("owner", 3u32, 3u32);
		let kitty_id = KittiesPallet::<T>::mint(&owner, Some([1u8; 16]), Some(pallet::Gender::Female)).unwrap();
	}: _(RawOrigin::Signed(owner), receiver.clone(), kitty_id)
	verify {
		// Compare the new owner
		let kitty = KittiesPallet::<T>::kitties(&kitty_id).unwrap();
		assert_eq!(kitty.owner, receiver);
	}

	buy_kitty {
		let s in 0 .. 10000;
		let buyer: T::AccountId = whitelisted_caller();
		let owner: T::AccountId = account("owner", 3u32, 3u32);
		let kitty_id = KittiesPallet::<T>::mint(&owner, Some([1u8; 16]), Some(pallet::Gender::Female)).unwrap();
		let _ = KittiesPallet::<T>::set_price(RawOrigin::Signed(owner.clone()).into(), kitty_id, Some(s.into()));
	}: {
		(RawOrigin::Signed(buyer), kitty_id, 0)
	}
	verify {
		// Compare the new owner
		let kitty = KittiesPallet::<T>::kitties(&kitty_id).unwrap();
		assert_eq!(kitty.owner, owner);
	}

	breed_kitty {
		let s in 0 .. 10000;
		let owner: T::AccountId = whitelisted_caller();
		let kitty_id_1 = KittiesPallet::<T>::mint(&owner, Some([1u8; 16]), Some(pallet::Gender::Female)).unwrap();
		let kitty_id_2 = KittiesPallet::<T>::mint(&owner, Some([2u8; 16]), Some(pallet::Gender::Male)).unwrap();
	}: _(RawOrigin::Signed(owner), kitty_id_1, kitty_id_2)

	impl_benchmark_test_suite!(KittiesPallet, crate::mock::new_test_ext_benchmark(1), crate::mock::Test);
}
