//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Template;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	create_kitty {
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))
	verify {
        let s_u64: u64 = s.into();
		assert_eq!(KittyCnt::<T>::get(), 1);
	}

	impl_benchmark_test_suite!(Template, crate::mock::new_test_ext_benchmark(), crate::mock::Test);
}
