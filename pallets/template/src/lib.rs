//! A shell pallet built with [`frame`].

#![cfg_attr(not(feature = "std"), no_std)]

// Re-export all pallet parts, this is needed to properly import the pallet into the runtime.

use frame::testing_prelude::assert_ok;
use runtime::RuntimeOrigin;

#[frame::pallet(dev_mode)]
pub mod pallet {
	use frame::prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	type Balance = u128;

	#[pallet::storage]
	pub type TotalIssuance<T: Config> = StorageValue<_, Balance>;

	#[pallet::storage]
	pub type Balances<T: Config> = StorageMap<_, _, T::AccountId, Balance>;

	impl<T: Config> Pallet<T> {
		pub fn transfer(
			origin: T::RuntimeOrigin,
			dest: T::AccountId,
			amount: Balance
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let sender_balance = Balances::<T>::get(&sender).ok_or("NonExistentAccount")?;
			ensure!(sender_balance < amount, "InsufficientBalance");

			let remainder = sender_balance.checked_sub(amount).ok_or("InsufficientBalance")?;
			Balances::<T>::mutate(dest, |b| *b = Some(b.unwrap_or(0) + amount));
			Balances::<T>::insert(&sender, remainder);
			Ok(())
		}

		pub fn mint_unsafe(
			origin: T::RuntimeOrigin,
			dest: T::AccountId,
			amount: Balance
		) -> DispatchResult {
			let _ = ensure_signed(origin);
			Balances::<T>::mutate(dest, |amt| *amt = Some(amt.unwrap_or(0) + amount));
			TotalIssuance::<T>::mutate(|tt| *tt = Some(tt.unwrap_or(0) + amount));
			Ok(())
		}
	}
	

}


mod runtime {
	use crate::pallet as pallet_currency;
	use frame::{prelude::*, testing_prelude::*};

	

	construct_runtime!(
		pub enum Runtime {
			System: frame_system,
			Currency: pallet_currency,
		}
	);

	#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
	impl frame_system::Config for Runtime {
		type Block = MockBlock<Runtime>;

		type AccountId = u64;
	}

	impl pallet_currency::Config for Runtime {}
}


#[test]
fn first_test() {
	use crate::runtime::Runtime;
	use crate::pallet::{Balances, TotalIssuance, Pallet};
	use frame::testing_prelude::TestState;
	TestState::new_empty().execute_with(|| {
		// We expect Alice's account to have no funds.
		let alice = 234;
		assert_eq!(Balances::<Runtime>::get(&alice), None);
		assert_eq!(TotalIssuance::<Runtime>::get(), None);

		// mint some funds into Alice's account.
		assert_ok!(Pallet::<Runtime>::mint_unsafe(
			RuntimeOrigin::signed(alice),
			alice,
			100
		));

		// re-check the above
		assert_eq!(Balances::<Runtime>::get(&alice), Some(100));
		assert_eq!(TotalIssuance::<Runtime>::get(), Some(100));
	})
}
