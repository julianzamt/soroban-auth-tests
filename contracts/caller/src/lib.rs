#![no_std]

use soroban_sdk::{contract, contractimpl, Address, Env};

mod callee {
    soroban_sdk::contractimport!(file = "../../target/wasm32-unknown-unknown/release/callee.wasm");
}

#[contract]
pub struct Caller;

#[contractimpl]
impl Caller {
    pub fn make_cross_call_caller_auth(env: Env, callee: Address, user: Address) -> u32 {
        user.require_auth();
        let client = callee::Client::new(&env, &callee);
        client.personal_counter(&user)
    }

    pub fn make_cross_call_no_caller_auth(env: Env, callee: Address, user: Address) -> u32 {
        let client = callee::Client::new(&env, &callee);
        client.personal_counter(&user)
    }
}

mod test;
