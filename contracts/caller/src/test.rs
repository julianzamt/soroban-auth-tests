#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, MockAuth, MockAuthInvoke},
    Address, Env, IntoVal,
};

extern crate std;

#[test]
fn with_caller_auth() {
    let env = Env::default();

    let callee = env.register_contract_wasm(None, callee::WASM);

    let user_1 = Address::generate(&env);

    let contract_id = env.register_contract(None, Caller);
    let client = CallerClient::new(&env, &contract_id);

    let counter = client
        .mock_auths(&[MockAuth {
            address: &user_1,
            invoke: &MockAuthInvoke {
                contract: &contract_id,
                fn_name: "make_cross_call_caller_auth",
                args: (&callee, &user_1).into_val(&env),
                sub_invokes: &[MockAuthInvoke {
                    contract: &callee,
                    fn_name: "personal_counter",
                    args: (&user_1,).into_val(&env),
                    sub_invokes: &[],
                }],
            },
        }])
        .make_cross_call_caller_auth(&callee, &user_1);

    std::println!("{:?}", env.auths());

    assert_eq!(counter, 1);
}

#[test]
fn without_caller_auth() {
    let env = Env::default();

    let callee = env.register_contract_wasm(None, callee::WASM);

    let user_1 = Address::generate(&env);

    let contract_id = env.register_contract(None, Caller);
    let client = CallerClient::new(&env, &contract_id);

    let counter = client
        .mock_auths(&[MockAuth {
            address: &user_1,
            invoke: &MockAuthInvoke {
                contract: &callee,
                fn_name: "personal_counter",
                args: (&user_1,).into_val(&env),
                sub_invokes: &[],
            },
        }])
        .make_cross_call_no_caller_auth(&callee, &user_1);

    std::println!("{:?}", env.auths());

    assert_eq!(counter, 1);
}
