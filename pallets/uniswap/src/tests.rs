use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};

fn genesis_config_works() {
	new_test_ext().execute_with(|| {
	});
}

#[test]
fn it_adds_liquidity() {
	new_test_ext().execute_with(|| {
		assert_eq!(Balances::free_balance(1), 10);
		assert_ok!(Uniswap::issue(Origin::signed(1), 100));
		assert_eq!(Uniswap::balance(0,1), 100);

		assert_ok!(Uniswap::add_liquidity(Origin::signed(1), 0, 10, 5));
		assert_eq!(Balances::free_balance(1), 5);
		assert_eq!(Uniswap::balance(0,1), 90);
		assert_eq!(Uniswap::balance(1,1), 15);

		assert_ok!(Uniswap::add_liquidity(Origin::signed(1), 0, 10, 5));
		assert_eq!(Balances::free_balance(1), 0);
		assert_eq!(Uniswap::balance(0,1), 80);
		//TODO not working fix this test, unsigned int underflow and end up 0
		assert_eq!(Uniswap::balance(1,1), 30);
	});
}

#[test]
fn it_swaps() {
	new_test_ext().execute_with(|| {
		//TODO good for now run more test with weirder numbers, make sure to check slippage
		hydrate_pools();
		assert_eq!(Balances::free_balance(1), 5);
		assert_ok!(Uniswap::swap_token_to_asset(Origin::signed(1), 0, 5));
		assert_eq!(Uniswap::balance(0,1), 85);
		assert_eq!(Balances::free_balance(1), 10);

	});
}

fn hydrate_pools() {
	assert_ok!(Uniswap::issue(Origin::signed(1), 100));
	assert_ok!(Uniswap::add_liquidity(Origin::signed(1), 0, 10, 5));
}

// #[test]
// // fn correct_error_for_none_value() {
// // 	new_test_ext().execute_with(|| {
// // 		// Ensure the expected error is thrown when no value is present.
// // 		assert_noop!(
// // 			Uniswap::cause_error(Origin::signed(1)),
// // 			Error::<Test>::NoneValue
// // 		);
// // 	});
// }



// #![cfg_attr(not(feature = "std"), no_std)]

// /// Edit this file to define custom logic or remove it if it is not needed.
// /// Learn more about FRAME and the core library of Substrate FRAME pallets:
// /// https://substrate.dev/docs/en/knowledgebase/runtime/frame

// use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get, Parameter};
// use frame_support::{
// 	weights::{Weight, GetDispatchInfo, Pays},
// 	traits::UnfilteredDispatchable,
// 	dispatch::DispatchResultWithPostInfo,
// };
// use frame_system::ensure_signed;

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

// /// Configure the pallet by specifying the parameters and types on which it depends.
// pub trait Trait: frame_system::Trait {
// 	/// Because this pallet emits events, it depends on the runtime's definition of an event.
// 	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
// 	type Call: Parameter + UnfilteredDispatchable<Origin=Self::Origin> + GetDispatchInfo;

// }

// // The pallet's runtime storage items.
// // https://substrate.dev/docs/en/knowledgebase/runtime/storage
// decl_storage! {
// 	// A unique name is used to ensure that the pallet's storage items are isolated.
// 	// This name may be updated, but each pallet in the runtime must use a unique name.
// 	// ---------------------------------vvvvvvvvvvvvvv
// 	trait Store for Module<T: Trait> as Uniswap {
// 		NativeToken get(fn native_token): u128;
// 		pub TokenBalances get(fn token_balances): map hasher(blake2_128_concat) u128 => u128;
// 		pub NativeTokenBalances get(fn native_token_balances): map hasher(blake2_128_concat) u128 => u128;
// 		TokenPairs get(fn token_pairs): u128;	
// 	}
// }

// // Pallets use events to inform users when important changes are made.
// // https://substrate.dev/docs/en/knowledgebase/runtime/events
// decl_event!(
// 	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
// 		/// Event documentation should end with an array that provides descriptive names for event
// 		/// parameters. [something, who]
// 		SomethingStored(u128, AccountId),
// 	}
// );

// // Errors inform users that something went wrong.
// decl_error! {
// 	pub enum Error for Module<T: Trait> {
// 		/// Error names should be descriptive.
// 		NoneValue,
// 		/// Errors should have helpful documentation associated with them.
// 		StorageOverflow,
// 		NotEmptyPool
// 	}
// }

// // Dispatchable functions allows users to interact with the pallet and invoke state changes.
// // These functions materialize as "extrinsics", which are often compared to transactions.
// // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
// decl_module! {
// 	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
// 		// Errors must be initialized if they are used by the pallet.
// 		type Error = Error<T>;

// 		// Events must be initialized if they are used by the pallet.
// 		fn deposit_event() = default;
		

// 		/// An example dispatchable that takes a singles value as a parameter, writes the value to
// 		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
// 		#[weight = 10_000 + T::DbWeight::get().writes(1)]
// 		pub fn add_liquidity(origin, call: Box<<T as Trait>::Call>, token_id: u128) -> dispatch::DispatchResult {
// 			// Check that the extrinsic was signed and get the signer.
// 			// This function will return an error if the extrinsic is not signed.
// 			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
// 			let who = ensure_signed(origin)?;

// 			let res = call.dispatch_bypass_filter(Assets::transfer());

// 			TokenBalances::insert(token_id, 0);
// 			NativeTokenBalances::insert(token_id, 0);

// 			// increment token pair count 


// 			// fix event signature
// 			Self::deposit_event(RawEvent::SomethingStored(token_id, who));
// 			// Return a successful DispatchResult
// 			Ok(())
// 		}


// 		/// An example dispatchable that may throw a custom error.
// 		#[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
// 		pub fn cause_error(origin) -> dispatch::DispatchResult {
// 			let _who = ensure_signed(origin)?;

// 		// 	// Read a value from storage.
// 		// 	match Something::get() {
// 		// 		// Return an error if the value has not been set.
// 		// 		None => Err(Error::<T>::NoneValue)?,
// 		// 		Some(old) => {
// 		// 			// Increment the value read from storage; will error in the event of overflow.
// 		// 			let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
// 		// 			// Update the value in storage with the incremented result.
// 		// 			Something::put(new);
// 		// 			Ok(())
// 		// 		},
// 		// 	}
// 			Ok(())
// 		}
// 	}
// }
