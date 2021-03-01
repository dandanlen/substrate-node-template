#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get, IterableStorageMap};
use frame_system::ensure_signed;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Config: frame_system::Config {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	// A unique name is used to ensure that the pallet's storage items are isolated.
	// This name may be updated, but each pallet in the runtime must use a unique name.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Config> as Toaster {
		// Learn more about declaring storage items:
		// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
		Toaster get(fn toaster): map hasher(identity) <T as frame_system::Config>::AccountId => ();

		Slices get(fn slices): u8;
	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SliceInserted(AccountId),
		Toasted(AccountId),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Config> {
		/// The account already has a slice in the toaster.
		ToastLimitReached,
		/// Can't add any more slices until slices are toasted.
		ToasterFull,
		/// No slice in the toaster aka. no skin in the game.
		NoSliceInTheToaster,
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		/// Insert a slice of toast.
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn insert_slice(origin) -> dispatch::DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;

			let num_slices = Slices::get();

			if num_slices == 4u8 {
				Err(Error::<T>::ToasterFull)?;
			}

			// Update storage.
			Toaster::<T>::try_mutate_exists(&who, |storage| {
				match storage {
					Some(_) => Err(Error::<T>::ToastLimitReached),
					None => {
						*storage = Some(());
						Ok(())
					},
				}
			})?;

			// If we get to here we can update the count.
			Slices::mutate(|count| *count += 1);

			// Emit an event.
			Self::deposit_event(RawEvent::SliceInserted(who));

			// Return a successful DispatchResult
			Ok(())
		}

		/// Toast the bread
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn toast(origin) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;

			if !Toaster::<T>::contains_key(who) {
				Err(Error::<T>::NoSliceInTheToaster)?;
			}

			let mut slices_removed = 0;
			Toaster::<T>::drain().for_each(|(acct, _)| {
				slices_removed += 1;
				Self::deposit_event(RawEvent::Toasted(acct));
			});
			Slices::mutate(|count| *count -= slices_removed);

			Ok(())
		}
	}
}
