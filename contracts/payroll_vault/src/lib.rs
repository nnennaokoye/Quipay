#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, token};

#[cfg(test)]
mod test;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
}

#[contract]
pub struct PayrollVault;

#[contractimpl]
impl PayrollVault {
    pub fn initialize(e: Env, admin: Address) {
        if e.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }
        e.storage().instance().set(&DataKey::Admin, &admin);
    }

    pub fn deposit(e: Env, from: Address, token: Address, amount: i128) {
        from.require_auth();
        if amount <= 0 {
            panic!("deposit amount must be positive");
        }
        let token_client = token::Client::new(&e, &token);
        token_client.transfer(&from, &e.current_contract_address(), &amount);
    }

    pub fn payout(e: Env, to: Address, token: Address, amount: i128) {
        let admin: Address = e.storage().instance().get(&DataKey::Admin).expect("not initialized");
        admin.require_auth();
        
        if amount <= 0 {
            panic!("payout amount must be positive");
        }

        let token_client = token::Client::new(&e, &token);
        token_client.transfer(&e.current_contract_address(), &to, &amount);
    }

    pub fn get_balance(e: Env, token: Address) -> i128 {
        let token_client = token::Client::new(&e, &token);
        token_client.balance(&e.current_contract_address())
    }
}
