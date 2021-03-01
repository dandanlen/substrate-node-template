use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};

#[test]
fn can_insert_slice() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(Toaster::insert_slice(Origin::signed(1)));
		// Read pallet storage and assert an expected result.
		assert_eq!(Toaster::slices(), 1u8);
	});
}

#[test]
fn cant_insert_more_than_four_slices() {
	new_test_ext().execute_with(|| {
		assert_ok!(Toaster::insert_slice(Origin::signed(1)));
		assert_ok!(Toaster::insert_slice(Origin::signed(2)));
		assert_ok!(Toaster::insert_slice(Origin::signed(3)));
		assert_ok!(Toaster::insert_slice(Origin::signed(4)));
		// Ensure toaster is full after four slices.
		assert_noop!(
			Toaster::insert_slice(Origin::signed(5)),
			Error::<Test>::ToasterFull
		);

		assert_eq!(Toaster::slices(), 4u8);
	});
}

#[test]
fn cant_insert_multiple_slices_per_user() {
	new_test_ext().execute_with(|| {
		assert_ok!(Toaster::insert_slice(Origin::signed(1)));
		// Ensure each user is limited to a single slice.
		assert_noop!(
			Toaster::insert_slice(Origin::signed(1)),
			Error::<Test>::ToastLimitReached
		);
	});
}

#[test]
fn only_slice_owners_can_toast() {
	new_test_ext().execute_with(|| {
		assert_ok!(Toaster::insert_slice(Origin::signed(1)));
		assert_ok!(Toaster::insert_slice(Origin::signed(2)));
		assert_ok!(Toaster::insert_slice(Origin::signed(3)));
		assert_ok!(Toaster::insert_slice(Origin::signed(4)));

		// Foreign account can't call toast
		assert_noop!(
			Toaster::toast(Origin::signed(5)),
			Error::<Test>::NoSliceInTheToaster
		);

		// Member can toast
		assert_ok!(Toaster::toast(Origin::signed(4)));

		// Toaster is emptied and accepting slices
		assert_eq!(Toaster::slices(), 0u8);
		assert_ok!(Toaster::insert_slice(Origin::signed(1)));
		assert_ok!(Toaster::insert_slice(Origin::signed(5)));
	});
}
