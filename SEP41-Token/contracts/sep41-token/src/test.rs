#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Env, String};

use crate::my_token::{Sep41Token, Sep41TokenClient};
struct SetUpResult<'a> {
    env: Env,
    client: Sep41TokenClient<'a>,
    sender: Address,
    receiver: Address,
}

fn setup<'a>() -> SetUpResult<'a> {
    let env = Env::default();

    let contract_id = env.register_contract(None, Sep41Token);

    let client = Sep41TokenClient::new(&env, &contract_id);

    let sender = Address::generate(&env);

    let receiver = Address::generate(&env);

    SetUpResult {
        env,
        client,
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
fn test_transfer() {
    let setup_result = setup();
}