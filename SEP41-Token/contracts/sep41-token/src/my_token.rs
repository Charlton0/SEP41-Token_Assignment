use soroban_sdk::{contract, contractimpl, Address, Env, IntoVal, String};

use crate::{
    error::ContractError,
    events::{Approval, Transfer},
    storage::{AllowanceKey, DataKey},
};

#[contract]
pub struct Sep41Token;

#[contractimpl]
impl Sep41Token {

    pub fn initialize(env: Env, admin: Address) {
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

    fn get_admin(env: &Env) -> Address {
    env.storage()
        .persistent()
        .get(&DataKey::Admin)
        .unwrap()
}
    pub fn balance(env: Env, id: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Balance(id))
            .unwrap_or(0)
    }

    pub fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Allowance(from.clone(), spender.clone()))
            .unwrap_or(0)
    }

    pub fn approve(
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

    pub fn transfer(
        env: &Env,
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
        .publish(env);

        Ok(())
    }

    pub fn total_supply(env: Env) -> i128 {
    env.storage()
        .persistent()
        .get(&DataKey::TotalSupply)
        .unwrap_or(0)
}

    pub fn decimals(_env: Env) -> u32 {
        18
    }

    pub fn name(env: Env) -> String {
        String::from_str(&env, "Sep41Token")
    }

    pub fn symbol(env: Env) -> String {
        String::from_str(&env, "SEP")
    }
}