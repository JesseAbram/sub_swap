#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{Parameter, decl_module, decl_event, decl_storage, decl_error, ensure, dispatch, debug};
use frame_support::{
		weights::{Weight, GetDispatchInfo, Pays},
		traits::{UnfilteredDispatchable, Currency, ReservableCurrency},
		dispatch::DispatchResultWithPostInfo,
	};
use sp_runtime::traits::{Member, AtLeast32Bit, AtLeast32BitUnsigned, Zero, StaticLookup};
use frame_system::ensure_signed;
use sp_runtime::traits::One;


#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;

/// The module configuration trait.
pub trait Trait: frame_system::Trait {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;

	/// The units in which we record balances.
	type Balance: Member + Parameter + AtLeast32BitUnsigned + Default + Copy;

	/// The arithmetic type of asset identifier.
	type AssetId: Parameter + AtLeast32Bit + Default + Copy;

	type Currency: ReservableCurrency<Self::AccountId>;

}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;

		fn deposit_event() = default;
		/// Issue a new class of fungible assets. There are, and will only ever be, `total`
		/// such assets and they'll all belong to the `origin` initially. It will have an
		/// identifier `AssetId` instance: this will be specified in the `Issued` event.
		///
		/// # <weight>
		/// - `O(1)`
		/// - 1 storage mutation (codec `O(1)`).
		/// - 2 storage writes (condec `O(1)`).
		/// - 1 event.
		/// # </weight>
		#[weight = 0]
		fn issue(origin, #[compact] total: <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance) {
			let origin = ensure_signed(origin)?;

			let id = Self::next_asset_id();
			<NextAssetId<T>>::mutate(|id| *id += One::one());

			<Balances<T>>::insert((id, &origin), total);
			<TotalSupply<T>>::insert(id, total);

			// Self::deposit_event(RawEvent::Issued(id, origin, total));
		}

		/// Move some assets from one holder to another.
		///
		/// # <weight>
		/// - `O(1)`
		/// - 1 static lookup
		/// - 2 storage mutations (codec `O(1)`).
		/// - 1 event.
		/// # </weight>
		#[weight = 0]
		fn transfer(origin,
			#[compact] id: T::AssetId,
			target: <T::Lookup as StaticLookup>::Source,
			#[compact] amount: <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance
		) {
			let origin = ensure_signed(origin)?;
			let origin_account = (id, origin.clone());
			let origin_balance = <Balances<T>>::get(&origin_account);
			let target = T::Lookup::lookup(target)?;
			ensure!(!amount.is_zero(), Error::<T>::AmountZero);
			ensure!(origin_balance >= amount, Error::<T>::BalanceLow);

			// Self::deposit_event(RawEvent::Transferred(id, origin, target.clone(), amount));
			<Balances<T>>::insert(origin_account, origin_balance - amount);
			<Balances<T>>::mutate((id, target), |balance| *balance += amount);
		}

		/// Destroy any assets of `id` owned by `origin`.
		///
		/// # <weight>
		/// - `O(1)`
		/// - 1 storage mutation (codec `O(1)`).
		/// - 1 storage deletion (codec `O(1)`).
		/// - 1 event.
		/// # </weight>
		#[weight = 0]
		fn destroy(origin, #[compact] id: T::AssetId) {
			let origin = ensure_signed(origin)?;
			let balance = <Balances<T>>::take((id, &origin));
			ensure!(!balance.is_zero(), Error::<T>::BalanceZero);

			<TotalSupply<T>>::mutate(id, |total_supply| *total_supply -= balance);
			// Self::deposit_event(RawEvent::Destroyed(id, origin, balance));
		}

	
		
	#[weight = 0]
	fn add_liquidity(
			origin,
			#[compact] id: T::AssetId,
			#[compact] asset_amount: <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance,
			native_amount: <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance
		) -> dispatch::DispatchResult {
			// handles deposit asset token
			let origin = ensure_signed(origin.clone())?;
			let origin_account = (id, origin.clone());
			let origin_balance = <Balances<T>>::get(&origin_account);
			ensure!(origin_balance >= asset_amount, Error::<T>::BalanceLow);	
			// deposit nativeToken
			T::Currency::slash(&origin, native_amount);

			let padding = 10000000.into();


			<Balances<T>>::mutate((id, &origin), |balance| *balance -= asset_amount);

			//TODO make one event for add liqudity
			// Self::deposit_event(RawEvent::Deposited(id, &origin, asset_amount));

			// update asset liquidity pool 
			<TokenBalances<T>>::mutate(id, |amount| *amount += asset_amount);

		
			// update nativetoken to token balance
			<NativeTokenBalances<T>>::mutate(id, |amount| *amount += native_amount);

			// return a token based on the % of the pool
			// map the token generated
			let total_token = <TokenBalances<T>>::get(id);
			let total_native_token = <NativeTokenBalances<T>>::get(id);
			let mut token_payout;

			// change if statment to see if mapping exists for token
			if <Exists<T>>::get(id) == false {
				token_payout = asset_amount + native_amount;
				// create token
				let liquidity_token_id = Self::next_asset_id();
				<NextAssetId<T>>::mutate(|id| *id += One::one());

				<Balances<T>>::insert((liquidity_token_id, &origin), token_payout);
				<TotalSupply<T>>::insert(liquidity_token_id, token_payout);
				// map it
				<LiquidityTokenTracker<T>>::insert(id, liquidity_token_id);
				<Exists<T>>::insert(id, true);
			} else {
				//math
				// get token as percent of tokens 
				// underflows always leads to zero
				// let percent_of_tokens_added = (asset_amount + native_amount) / (((total_token) + (total_native_token))
				let percent_of_tokens_added = ((asset_amount + native_amount) * padding) / ((total_token - asset_amount) + (total_native_token - native_amount));
				

				// get total current supply of LToken
				let liquidity_token_id = <LiquidityTokenTracker<T>>::get(id);
				let total_liq_token = <TotalSupply<T>>::get(liquidity_token_id);
				// token to issue 
				token_payout = percent_of_tokens_added * total_liq_token;
				token_payout = token_payout / padding;
				// mint it
				//TODO check overflow?
				//TODO change to token payout when fix overflow  (instead of percent tokens added) 
				<Balances<T>>::mutate((liquidity_token_id, &origin), |balance| *balance += token_payout);
				<TotalSupply<T>>::mutate(liquidity_token_id, |supply| *supply += token_payout);

			}
			 

			
			Ok(())
		}

	#[weight = 0]
	fn remove_liquidity(
				origin,
				#[compact] id: T::AssetId,
				#[compact] asset_amount: <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance,
			) -> dispatch::DispatchResult {
				
		//Take liquidity token return corresponding pool values
		Ok(())
	}
	#[weight = 0]
	fn swap_token_to_asset(
				origin,
				#[compact] id: T::AssetId,
				#[compact] asset_amount: <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance,
			) -> dispatch::DispatchResult {
				let who = ensure_signed(origin)?;
				let total_token = <TokenBalances<T>>::get(id);
				let total_native_token = <NativeTokenBalances<T>>::get(id);
				let padding = 10000000.into();
				// underflow issue
				let multiplier =  (total_native_token * padding) / (total_token - asset_amount);
				let mut payable_value = asset_amount * multiplier;
				payable_value = payable_value / padding;
				T::Currency::deposit_into_existing(&who, payable_value)?;
				<Balances<T>>::mutate((id, &who), |balance| *balance -= asset_amount);
				<TokenBalances<T>>::mutate(id, |amount| *amount += asset_amount);
				<NativeTokenBalances<T>>::mutate(id, |amount| *amount -= payable_value);

		Ok(())
	}

	#[weight = 0]
	fn swap_asset_to_token(
				origin,
				#[compact] id: T::AssetId,
				#[compact] asset_amount: <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance,
			) -> dispatch::DispatchResult {
		//Takes in asset returns native token
		Ok(())
	}

	#[weight = 0]
	fn swap_token_to_token(
				origin,
				#[compact] id1: T::AssetId,
				#[compact] id2: T::AssetId,
			) -> dispatch::DispatchResult {
		//Take in token turns into native asset turns again into new asset 
		Ok(())
	}

	}
}

decl_event! {
	pub enum Event<T> where
		<T as frame_system::Trait>::AccountId,
		<T as Trait>::Balance,
		<T as Trait>::AssetId,
	{
		/// Some assets were issued. [asset_id, owner, total_supply]
		Issued(AssetId, AccountId, Balance),
		/// Some assets were transferred. [asset_id, from, to, amount]
		Transferred(AssetId, AccountId, AccountId, Balance),
		/// Some assets were destroyed. [asset_id, owner, balance]
		Destroyed(AssetId, AccountId, Balance),
		Deposited(AssetId, AccountId, Balance),
	}
}

decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Transfer amount should be non-zero
		AmountZero,
		/// Account balance must be greater than or equal to the transfer amount
		BalanceLow,
		/// Balance should be non-zero
		BalanceZero,
	}
}

decl_storage! {
	trait Store for Module<T: Trait> as Uniswap {
		/// The number of units of assets held by any given account.
		Balances: map hasher(blake2_128_concat) (T::AssetId, T::AccountId) => <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;
		/// The next asset identifier up for grabs.
		NextAssetId get(fn next_asset_id): T::AssetId;
		/// The total unit supply of an asset.
		/// TWOX-NOTE: `AssetId` is trusted, so this is safe.
		TotalSupply: map hasher(twox_64_concat) T::AssetId => <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;

		//TODO fix o they are generic types
		pub TokenBalances: map hasher(blake2_128_concat) T::AssetId => <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;
		pub NativeTokenBalances: map hasher(blake2_128_concat) T::AssetId => <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;
		pub LiquidityTokenTracker: map hasher(blake2_128_concat) T::AssetId => T::AssetId;
		pub Exists: map hasher(blake2_128_concat) T::AssetId => bool;


	}
}

// The main implementation block for the module.
impl<T: Trait> Module<T> {
	// Public immutables

	/// Get the asset `id` balance of `who`.
	pub fn balance(id: T::AssetId, who: T::AccountId) -> <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance {
		<Balances<T>>::get((id, who))
	}

	/// Get the total supply of an asset `id`.
	pub fn total_supply(id: T::AssetId) -> <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance {
		<TotalSupply<T>>::get(id)
	}
	pub fn token_balance(id: T::AssetId) -> <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance {
		<TokenBalances<T>>::get(id)
	}

	pub fn native_token_balance(id: T::AssetId) -> <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance {
		<NativeTokenBalances<T>>::get(id)
	}
	pub fn liquidity_token_tracker(id: T::AssetId) -> T::AssetId {
		<LiquidityTokenTracker<T>>::get(id)
	}
	pub fn exists(id: T::AssetId) -> bool {
		<Exists<T>>::get(id)
	}
}