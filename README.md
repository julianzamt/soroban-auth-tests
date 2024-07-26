# soroban-auth-tests
A minimal example of how auth works on Soroban's cross calls

Requirements:
    * [stellar-cli](https://developers.stellar.org/docs/build/smart-contracts/getting-started/setup) and its dependencies

To build:
`stellar contract build` or its alias `soroban contract build`

To test:
`cargo test` or `cargo test -- --nocapture` to see the auth tree logs
