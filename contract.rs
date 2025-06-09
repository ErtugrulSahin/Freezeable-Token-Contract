use soroban_sdk::{contractimpl, Address, Env, Symbol, Map, contracttype};

pub struct TokenContract;

#[contracttype]
pub struct DataKey;

#[contractimpl]
impl TokenContract {
    // Storage: Map<account, bool> indicating if the account is frozen
    fn frozen_accounts<'a>(env: &'a Env) -> Map<'a, Address, bool> {
        env.storage().instance().get_map(Symbol::short("frozen"))
    }

    // Freeze the specified account (only the account owner can freeze themselves)
    pub fn freeze_account(env: Env, account: Address) {
        let caller = env.invoker();
        assert_eq!(caller, account, "Only the account owner can freeze their account.");
        let mut frozen = Self::frozen_accounts(&env);
        frozen.set(account.clone(), true);
        env.storage().instance().set(Symbol::short("frozen"), &frozen);
    }

    // Unfreeze the specified account (only the account owner can unfreeze themselves)
    pub fn unfreeze_account(env: Env, account: Address) {
        let caller = env.invoker();
        assert_eq!(caller, account, "Only the account owner can unfreeze their account.");
        let mut frozen = Self::frozen_accounts(&env);
        frozen.set(account.clone(), false);
        env.storage().instance().set(Symbol::short("frozen"), &frozen);
    }

    // Example transfer function with freeze check
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        let frozen = Self::frozen_accounts(&env);
        if let Some(true) = frozen.get(from.clone()) {
            panic!("Account is frozen. Transfer not allowed.");
        }
        // ... (rest of the transfer logic would go here)
    }
}

