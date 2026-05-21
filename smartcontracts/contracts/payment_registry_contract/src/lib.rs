#![no_std]

use soroban_sdk::{contract, contractimpl, Env};

mod logic;
mod storage;
mod types;

#[contract]
pub struct PaymentRegistryContract;

#[contractimpl]
impl PaymentRegistryContract {
    pub fn version(_env: Env) -> u32 {
        1
    }

    pub fn create_payment_record(
        env: Env,
        merchant_address: soroban_sdk::Address,
        amount: i128,
        asset: soroban_sdk::Address,
    ) -> Result<soroban_sdk::String, crate::types::Error> {
        logic::create_payment_record(&env, merchant_address, amount, asset)
    }
}
