use crate::{mock::*, Error};
use frame_support::{
    assert_noop, assert_ok,
};

#[test]
fn should_create_kitty() {
	new_test_ext().execute_with(|| {
	// Dispatch a signed extrinsic.
        let origin = Origin::signed(1);
        assert_ok!(KittiesModule::create_kitty(origin));
	});
}

#[test]
fn owner_can_set_price_for_kitty() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
        let account_a = 1;
        assert_ok!(KittiesModule::create_kitty(Origin::signed(account_a)));

        // Get new create kitties
        let x = KittiesModule::kitties_owned(account_a);
        let kitty_id = x.get(0).unwrap();
        
        // Set price
        assert_ok!(KittiesModule::set_price(Origin::signed(account_a), *kitty_id, Some(10)));
	});
}

#[test]
fn user_can_not_set_price_for_kitty_of_other() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
        let account_a = 1;
        let account_b = 2;
        assert_ok!(KittiesModule::create_kitty(Origin::signed(account_a)));

        // Get new create kitties
        let x = KittiesModule::kitties_owned(account_a);
        let kitty_id = x.get(0).unwrap();
        
        // Set price
        assert_noop!(
            KittiesModule::set_price(Origin::signed(account_b), *kitty_id, Some(10)),
            Error::<Test>::NotKittyOwner
        );
	});
}

#[test]
fn should_buy_kitty() {
	new_test_ext().execute_with(|| {
        let account_a = 1;
        let account_b = 2;

        // Dispatch a signed extrinsic.
        
        // Account A create and set price for kitty
        assert_ok!(KittiesModule::create_kitty(Origin::signed(account_a)));
        let x = KittiesModule::kitties_owned(account_a);
        let account_a_kitty_id = x.get(0).unwrap().clone();
        assert_ok!(KittiesModule::set_price(Origin::signed(account_a), account_a_kitty_id, Some(10)));

        // Account B create and set price for kitty
        assert_ok!(KittiesModule::create_kitty(Origin::signed(account_b)));
        let x = KittiesModule::kitties_owned(account_b);
        let account_b_kitty_id = x.get(0).unwrap().clone();
        assert_ok!(KittiesModule::set_price(Origin::signed(account_b), account_b_kitty_id, Some(10)));

        // Account A buy Account B's kity
        assert_ok!(KittiesModule::buy_kitty(Origin::signed(account_a), account_b_kitty_id, 10));
	});
}

#[test]
fn should_breed_kitty() {
	new_test_ext().execute_with(|| {
        let account_a = 1;
        // Account A create and set price for 2 kitties
        assert_ok!(KittiesModule::create_kitty(Origin::signed(account_a)));
        // Update block number to 100 to prevent random the same Kitty sex
        System::set_block_number(100);
        assert_ok!(KittiesModule::create_kitty(Origin::signed(account_a)));

        let kitties_vec = KittiesModule::kitties_owned(account_a);
        let account_a_kitty_id_1 = kitties_vec.get(0).unwrap().clone();
        let account_a_kitty_id_2 = kitties_vec.get(1).unwrap().clone();

        assert_ok!(KittiesModule::breed_kitty(Origin::signed(account_a), account_a_kitty_id_1, account_a_kitty_id_2));
	});
}

#[test]
fn should_transfer_kitty() {
	new_test_ext().execute_with(|| {
        let account_a = 1;
        let account_b = 2;

        // Dispatch a signed extrinsic.
        
        // Account A create and set price for kitty
        assert_ok!(KittiesModule::create_kitty(Origin::signed(account_a)));
        let x = KittiesModule::kitties_owned(account_a);
        let account_a_kitty_id = x.get(0).unwrap().clone();

        assert_ok!(KittiesModule::transfer(Origin::signed(account_a), account_b, account_a_kitty_id));
	});
}
