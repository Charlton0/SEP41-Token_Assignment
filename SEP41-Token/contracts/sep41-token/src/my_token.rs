use soroban_sdk::{contract, contractimpl, Address, Env, IntoVal, String};

use crate::{
    error::ContractError,
    events::{Approval, Transfer},
    storage::DataKey,
};

#[contract]
pub struct Sep41Token;

#[contractimpl]
impl crate::token_trait::TokenInterface for Sep41Token {

    fn initialize(env: Env, admin: Address) {
    if env
        .storage()
        .persistent()
        .has(&DataKey::Admin)
    {
        panic!("already initialized");
    }

    env.storage()
        .persistent()
        .set(&DataKey::Admin, &admin);

    env.storage()
        .persistent()
        .set(&DataKey::TotalSupply, &0i128);
}


    fn balance(env: Env, id: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Balance(id))
            .unwrap_or(0)
    }

    fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Allowance(from.clone(), spender.clone()))
            .unwrap_or(0)
    }

    fn approve(
        env: Env,
        from: Address,
        spender: Address,
        amount: i128,
        live_until_ledger: u32,
    ) -> Result<(), ContractError> {
        from.require_auth();

        let from_balance = Self::balance(env.clone(), from.clone());

        if from_balance < amount {
            return Err(ContractError::InsufficientFunds);
        }

        let key = DataKey::Allowance(from.clone(), spender.clone());
        env.storage().persistent().set(&key, &amount);

        Approval {
            from,
            spender,
            amount: amount.try_into().unwrap(),
            live_until_ledger: live_until_ledger.into_val(&env),
        }
        .publish(&env);

        Ok(())
    }

    fn transfer(
        env: Env,
        from: Address,
        to: Address,
        amount: i128,
    ) -> Result<(), ContractError> {
        from.require_auth();
        let sender_balance = Self::balance(env.clone(), from.clone());

        let receiver_balance = Self::balance(env.clone(), to.clone());

        if sender_balance < amount {
            return Err(ContractError::InsufficientFunds);
        }

       env.storage().persistent().set(
    &DataKey::Balance(from.clone()),
    &(sender_balance - amount),
);

env.storage().persistent().set(
    &DataKey::Balance(to.clone()),
    &(receiver_balance + amount),
);

        Transfer {
            from,
            to,
            amount: amount.try_into().unwrap(),
        }
        .publish(&env);

        Ok(())
    }

    fn total_supply(env: Env) -> i128 {
    env.storage()
        .persistent()
        .get(&DataKey::TotalSupply)
        .unwrap_or(0)
}

//Mint function creates new token and assign to user, this can only be done by admin
fn mint(
    env: Env,
    admin: Address,
    to: Address,
    amount: i128,
) -> Result<(), ContractError> {
    admin.require_auth();

    if amount <= 0 {
        return Err(ContractError::InvalidAmount);
    }

    let mut supply: i128 = env
        .storage()
        .persistent()
        .get(&DataKey::TotalSupply)
        .unwrap_or(0);

    let balance = Self::balance(env.clone(), to.clone());

    env.storage().persistent().set(
        &DataKey::Balance(to.clone()),
        &(balance + amount),
    );

    env.storage().persistent().set(
        &DataKey::TotalSupply,
        &(supply + amount),
    );

    Ok(())
}

//Burn function destroys tokens from an address, performed by admin or owner
fn burn(
    env: Env,
    from: Address,
    amount: i128,
) -> Result<(), ContractError> {
    from.require_auth();

    if amount <= 0 {
        return Err(ContractError::InvalidAmount);
    }

    let balance = Self::balance(env.clone(), from.clone());

    if balance < amount {
        return Err(ContractError::InsufficientFunds);
    }

    let mut supply: i128 = env
        .storage()
        .persistent()
        .get(&DataKey::TotalSupply)
        .unwrap_or(0);

    env.storage().persistent().set(
        &DataKey::Balance(from.clone()),
        &(balance - amount),
    );

    env.storage().persistent().set(
        &DataKey::TotalSupply,
        &(supply - amount),
    );

    Ok(())
}

//This function allows Spender to move tokens on behalf of owner.
fn transfer_from(
    env: Env,
    spender: Address,
    from: Address,
    to: Address,
    amount: i128,
) -> Result<(), ContractError> {
    spender.require_auth();

    if amount <= 0 {
        return Err(ContractError::InvalidAmount);
    }

    let allowance = Self::allowance(env.clone(), from.clone(), spender.clone());

    if allowance < amount {
        return Err(ContractError::InsufficientAllowance);
    }

    let from_balance = Self::balance(env.clone(), from.clone());
    let to_balance = Self::balance(env.clone(), to.clone());

    if from_balance < amount {
        return Err(ContractError::InsufficientFunds);
    }

    // update balances
    env.storage().persistent().set(
        &DataKey::Balance(from.clone()),
        &(from_balance - amount),
    );

    env.storage().persistent().set(
        &DataKey::Balance(to.clone()),
        &(to_balance + amount),
    );

    // reduce allowance
    env.storage().persistent().set(
        &DataKey::Allowance(from.clone(), spender.clone()),
        &(allowance - amount),
    );

    Ok(())
}

//this fuction allows spender to burn tokens from owner
fn burn_from(
    env: Env,
    spender: Address,
    from: Address,
    amount: i128,
) -> Result<(), ContractError> {
    spender.require_auth();

    if amount <= 0 {
        return Err(ContractError::InvalidAmount);
    }

    let allowance = Self::allowance(env.clone(), from.clone(), spender.clone());

    if allowance < amount {
        return Err(ContractError::InsufficientAllowance);
    }

    let balance = Self::balance(env.clone(), from.clone());

    if balance < amount {
        return Err(ContractError::InsufficientFunds);
    }

    let mut supply: i128 = env
        .storage()
        .persistent()
        .get(&DataKey::TotalSupply)
        .unwrap_or(0);

    // update state
    env.storage().persistent().set(
        &DataKey::Balance(from.clone()),
        &(balance - amount),
    );

    env.storage().persistent().set(
        &DataKey::TotalSupply,
        &(supply - amount),
    );

    env.storage().persistent().set(
        &DataKey::Allowance(from.clone(), spender.clone()),
        &(allowance - amount),
    );

    Ok(())
}

    fn decimals(_env: Env) -> u32 {
        18
    }

    fn name(env: Env) -> String {
        String::from_str(&env, "Sep41Token")
    }

    fn symbol(env: Env) -> String {
        String::from_str(&env, "SEP")
    }
}

//Helper function
impl Sep41Token {
    fn get_admin(env: &Env) -> Address {
    env.storage()
        .persistent()
        .get(&DataKey::Admin)
        .unwrap()
}
}