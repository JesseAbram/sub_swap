use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};

#[test]
fn it_adds_a_pair() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(Uniswap::add_pair(Origin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		assert_eq!(Uniswap::token_balances(42), 0);
	});
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
