use soroban_sdk::{Address, Env, String};

pub trait TokenInterface {
    fn initialize(env: Env, admin: Address);

    fn balance(env: Env, id: Address) -> i128;

    fn allowance(env: Env, from: Address, spender: Address) -> i128;

    fn approve(
        env: Env,
        from: Address,
        spender: Address,
        amount: i128,
        live_until_ledger: u32,
    ) -> Result<(), crate::error::ContractError>;

    fn transfer(
        env: Env,
        from: Address,
        to: Address,
        amount: i128,
    ) -> Result<(), crate::error::ContractError>;

    fn transfer_from(
        env: Env,
        spender: Address,
        from: Address,
        to: Address,
        amount: i128,
    ) -> Result<(), crate::error::ContractError>;

    fn burn(
        env: Env,
        from: Address,
        amount: i128,
    ) -> Result<(), crate::error::ContractError>;

    fn burn_from(
        env: Env,
        spender: Address,
        from: Address,
        amount: i128,
    ) -> Result<(), crate::error::ContractError>;

    fn mint(env: Env, to: Address, amount: i128) -> Result<(), crate::error::ContractError>;

    fn total_supply(env: Env) -> i128;

    fn decimals(env: Env) -> u32;

    fn name(env: Env) -> String;

    fn symbol(env: Env) -> String;
}