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
		/// Storage Map to store merkle root of the data points of a file stored in IPFS
		MerkleRoots get(fn merkle_root): map hasher(blake2_128_concat)  Vec<u8> => (T::AccountId, T::BlockNumber);
	}
}

decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as frame_system::Config>::AccountId,
	{
		/// Event emitted when merkle root is stored.
		MerkleRootEntry(AccountId, Vec<u8>),

		/// Event emitted when account id and block number is accessed using merkle root
		MerkleRootEntryFound(AccountId, Vec<u8>),
	}
);

decl_error! {
	pub enum Error for Module<T: Config> {
		/// Error thrown when no entry of the merkle root is found
		NoValueStored,
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {

		// Initialize errors
		type Error = Error<T>;

		// Initialize events
		fn deposit_event() = default;

		/// Store the account id and blocknumber set at a particular key as merkle root
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