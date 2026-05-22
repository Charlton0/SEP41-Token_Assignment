#![no_std]
// use soroban_sdk::{contract, contractimpl, vec, Env, String, Vec};

// #[contract]
// pub struct Contract;


// #[contractimpl]
// impl Contract {
//     pub fn hello(env: Env, to: String) -> Vec<String> {
//         vec![&env, String::from_str(&env, "Hello"), to]
//     }
// }
mod error;
mod events;
mod my_token;
mod storage;
mod test;
mod token_trait;
