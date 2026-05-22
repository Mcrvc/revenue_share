#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Map};

const TOTAL_FUNDS: soroban_sdk::Symbol = symbol_short!("Total");
const SHARES: soroban_sdk::Symbol = symbol_short!("Shares");
const REVENUE: soroban_sdk::Symbol = symbol_short!("Revenue");

#[contract]
pub struct RevenueShareContract;

#[contractimpl]
impl RevenueShareContract {
    pub fn contribute(env: Env, contributor: Address, amount: i128) {
        contributor.require_auth();

        let mut shares: Map<Address, i128> = env.storage().instance().get(&SHARES).unwrap();
        let current_share = shares.get(contributor.clone()).unwrap_or(0);
        
        shares.set(contributor, current_share + amount);
        env.storage().instance().set(&SHARES, &shares);

        let total: i128 = env.storage().instance().get(&TOTAL_FUNDS).unwrap();
        env.storage().instance().set(&TOTAL_FUNDS, &(total + amount));
    }

    pub fn update_revenue(env: Env, amount: i128) {
        let current_revenue: i128 = env.storage().instance().get(&REVENUE).unwrap_or(0);
        env.storage().instance().set(&REVENUE, &(current_revenue + amount));
    }
    
    pub fn claim_reward(env: Env, contributor: Address) -> i128 {
        contributor.require_auth();

        let shares: Map<Address, i128> = env.storage().instance().get(&SHARES).unwrap();
        let my_contribution = shares.get(contributor.clone()).unwrap_or(0);
        
        let total_contributed: i128 = env.storage().instance().get(&TOTAL_FUNDS).unwrap();
        let total_revenue: i128 = env.storage().instance().get(&REVENUE).unwrap();

        if my_contribution == 0 || total_contributed == 0 || total_revenue == 0 {
            return 0;
        }

        let my_reward = (my_contribution * total_revenue) / total_contributed;
        my_reward
    }

    pub fn preview_reward(env: Env, contributor: Address) -> i128 {
        let shares: Map<Address, i128> = env.storage().instance().get(&SHARES).unwrap_or(Map::new(&env));
        let my_contribution = shares.get(contributor.clone()).unwrap_or(0);

        let total_contributed: i128 = env.storage().instance().get(&TOTAL_FUNDS).unwrap_or(0);
        let total_revenue: i128 = env.storage().instance().get(&REVENUE).unwrap_or(0);

        if my_contribution == 0 || total_contributed == 0 || total_revenue == 0 {
            return 0;
        }

        (my_contribution * total_revenue) / total_contributed
    }
}
