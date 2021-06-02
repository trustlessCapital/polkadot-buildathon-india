//! A pallet to demonstrate usage of a simple storage map
//!
//! Storage maps map a key type to a value type. The hasher used to hash the key can be customized.
//! This pallet uses the `blake2_128_concat` hasher. This is a good default hasher.

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	decl_error, decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure, StorageMap, traits::Vec,
};
use frame_system::ensure_signed;


#[cfg(test)]
mod tests;

pub trait Config: frame_system::Config {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

decl_storage! {
	trait Store for Module<T: Config> as TemplateModule {
		MerkleRoots get(fn merkle_root): map hasher(blake2_128_concat)  Vec<u8> => (T::AccountId, T::BlockNumber);
	}
}

decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as frame_system::Config>::AccountId,
	{
		MerkleRootEntry(AccountId, Vec<u8>),

		/// A user has read their entry, leaving it in storage
		MerkleRootEntryFound(AccountId, Vec<u8>),
	}
);

decl_error! {
	pub enum Error for Module<T: Config> {
		/// The requested user has not stored a value yet
		NoValueStored,
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {

		// Initialize errors
		type Error = Error<T>;

		// Initialize events
		fn deposit_event() = default;

		/// Set the merkle root stored at a particular key
		#[weight = 10_000]
		fn set_merkleroot_entry(origin, entry: Vec<u8>) -> DispatchResult {
			// A user can only set their own entry
			let user = ensure_signed(origin)?;

			let current_block = <frame_system::Module<T>>::block_number();

			<MerkleRoots<T>>::insert(&entry, (&user, current_block));

			Self::deposit_event(RawEvent::MerkleRootEntry(user, entry));
			Ok(())
		}

		/// Read the value stored at a particular key and emit it in an event
		#[weight = 10_000]
		fn get_merkleroot(origin, entry: Vec<u8>) -> DispatchResult {
			// Any user can get any other user's entry
			let getter = ensure_signed(origin)?;

			ensure!(<MerkleRoots<T>>::contains_key(&entry), Error::<T>::NoValueStored);
			let _merkleroot = <MerkleRoots<T>>::get(entry.clone());
			Self::deposit_event(RawEvent::MerkleRootEntryFound(getter, entry.clone()));
			Ok(())
		}
	}
}