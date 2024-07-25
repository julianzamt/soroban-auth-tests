#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env};

#[contract]
pub struct Callee;

#[contractimpl]
impl Callee {
    pub fn personal_counter(env: Env, user: Address) -> u32 {
        user.require_auth();
        
        let mut counter: u32 = env.storage().persistent().get(&user).unwrap_or(0);

        counter += 1;

        env.storage().persistent().set(&user, &counter);

        counter
    }
}

mod test;
