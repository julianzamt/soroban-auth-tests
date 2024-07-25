#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn test() {
    let env = Env::default();
    env.mock_all_auths();

    let user_1 = Address::generate(&env);

    let contract_id = env.register_contract(None, Callee);
    let client = CalleeClient::new(&env, &contract_id);

    assert_eq!(client.personal_counter(&user_1), 1);
    assert_eq!(client.personal_counter(&user_1), 2);
    assert_eq!(client.personal_counter(&user_1), 3);
}
