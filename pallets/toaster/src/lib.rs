#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use frame_support::{decl_module, decl_storage, decl_event, decl_error, ensure, dispatch, traits::Get};
use frame_system::ensure_signed;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

const MAX_SLOTS:u8 = 4;

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
		NumberOf get(fn number_of): u8;
		ToasterSlots get(fn toaster_slots): map hasher(blake2_128_concat) T::AccountId => ();
	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
		/// Toast Added by [who]
		ToastAdded(AccountId),
		/// Congratulations we have toast for all [everyone]
		Toasted(AccountId),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Config> {
		/// The toaster is full I am afraid
		ToasterFull,
		/// Already toasting greedy
		Greedy,
		/// Missing some toast
		ToasterNotReady
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

		/// Add my toast
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn add_my_toast(origin) {
			let who = ensure_signed(origin)?;
			// We only have 4 slots
			ensure!(NumberOf::get() != MAX_SLOTS, Error::<T>::ToasterFull);
			// If I have already added my toast then return error Greedy
			ensure!(!ToasterSlots::<T>::contains_key(&who), Error::<T>::Greedy);
			// Insert my toast
			ToasterSlots::<T>::insert(who.clone(), ());
			// Increase counter
			NumberOf::mutate(|v| *v = *v + 1);
			// Send event
			Self::deposit_event(RawEvent::ToastAdded(who));
		}

		/// Toast the lot
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn toast_me(origin) {
			// Only if full else return error ToasterNotReady
			ensure!(NumberOf::get() == MAX_SLOTS, Error::<T>::ToasterNotReady);
			// Clear map on each iteration send event
			for who in ToasterSlots::<T>::drain() {
				Self::deposit_event(RawEvent::ToastAdded(who.0));
			}
			// Reset counter
			NumberOf::put(0);
		}
	}
}
