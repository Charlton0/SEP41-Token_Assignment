#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Env, String};

use crate::my_token::{Sep41Token, Sep41TokenClient};
struct SetUpResult<'a> {
    env: Env,
    client: Sep41TokenClient<'a>,
    admin: Address,
    sender: Address,
    receiver: Address,
}

fn setup<'a>() -> SetUpResult<'a> {
    let env = Env::default();

    let contract_id = env.register_contract(None, Sep41Token);

    let client = Sep41TokenClient::new(&env, &contract_id);

    let admin = Address::generate(&env);

    let sender = Address::generate(&env);

    let receiver = Address::generate(&env);

    client.initialize(&admin);

    client.mint(&admin, &sender, &1000);

    SetUpResult {
        env,
        client,
        admin,
        sender,
        receiver,
    }
}

#[test]
fn test_name() {
    let setup_result = setup();

    let name = setup_result.client.name();
    let token_name = String::from_str(&setup_result.env, "Sep41Token");
    assert_eq!(name, token_name);
}

#[test]
fn test_symbol() {
    let setup_result = setup();

    let name = setup_result.client.symbol();
    let token_name = String::from_str(&setup_result.env, "SEP");

    let not_token_name = String::from_str(&setup_result.env, "Sep");
    assert_eq!(name, token_name);
    assert_ne!(name, not_token_name);
}

#[test]
fn test_decimal() {
    let setup_result = setup();

    let decimal = setup_result.client.decimals();
    let token_decimal = 18;

    assert_eq!(decimal, token_decimal);
}


#[test]
fn test_mint() {
    let setup_result = setup();

    setup_result.env.mock_all_auths();

    setup_result.
     client.mint(
    &setup_result.admin,
    &setup_result.receiver,
    &500,
);
    let balance = setup_result
        .client
        .balance(&setup_result.receiver);

    assert_eq!(balance, 500);
}

#[test]
fn test_transfer() {
    let setup_result = setup();

    setup_result.client.transfer(
        &setup_result.sender,
        &setup_result.receiver,
        &200,
    );

    let sender_balance = setup_result
        .client
        .balance(&setup_result.sender);

    let receiver_balance = setup_result
        .client
        .balance(&setup_result.receiver);

    assert_eq!(sender_balance, 800);
    assert_eq!(receiver_balance, 200);
}

#[test]
fn test_approve_and_allowance() {
    let setup_result = setup();

    setup_result.client.approve(
        &setup_result.sender,
        &setup_result.receiver,
        &300,
        &1000,
    );

    let allowance = setup_result.client.allowance(
        &setup_result.sender,
        &setup_result.receiver,
    );

    assert_eq!(allowance, 300);
}

#[test]
fn test_transfer_from() {
    let setup_result = setup();

    setup_result.client.approve(
        &setup_result.sender,
        &setup_result.receiver,
        &300,
        &1000,
    );

    setup_result.client.transfer_from(
        &setup_result.receiver,
        &setup_result.sender,
        &setup_result.receiver,
        &200,
    );

    let sender_balance = setup_result
        .client
        .balance(&setup_result.sender);

    let receiver_balance = setup_result
        .client
        .balance(&setup_result.receiver);

    assert_eq!(sender_balance, 800);
    assert_eq!(receiver_balance, 200);
}

#[test]
fn test_burn() {
    let setup_result = setup();

    setup_result.client.burn(
        &setup_result.sender,
        &300,
    );

    let balance = setup_result
        .client
        .balance(&setup_result.sender);

    assert_eq!(balance, 700);
}

#[test]
#[should_panic]
fn test_unauthorized_mint() {
    let setup_result = setup();

    setup_result.client.mint(
        &setup_result.receiver,
        &100,
    );
}