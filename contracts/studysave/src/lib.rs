#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype,
    token, Address, Env,
};

// ─── Storage Keys ─────────────────────────────────────────────────────────────

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Vault(Address),     // full vault record keyed by student's address
    UsdcToken,          // address of the USDC token contract on Stellar
}

// ─── Vault Record ─────────────────────────────────────────────────────────────

#[contracttype]
#[derive(Clone)]
pub struct Vault {
    pub student: Address,
    pub balance: i128,        // USDC deposited, in stroops (1 USDC = 10_000_000 stroops)
    pub unlock_timestamp: u64, // Unix timestamp; withdrawal blocked before this
    pub is_open: bool,        // false once withdrawn or never opened
}

// ─── Contract ─────────────────────────────────────────────────────────────────

#[contract]
pub struct StudySave;

#[contractimpl]
impl StudySave {
    /// One-time initialization: store the USDC token contract address.
    /// Must be called by the deployer immediately after deployment.
    pub fn init(env: Env, usdc_token: Address) {
        // Guard: do not allow re-initialization
        if env.storage().instance().has(&DataKey::UsdcToken) {
            panic!("contract already initialized");
        }
        env.storage()
            .instance()
            .set(&DataKey::UsdcToken, &usdc_token);
    }

    /// Student opens a vault and deposits USDC in a single step.
    ///
    /// Flow:
    ///   1. Student authorizes this call (require_auth).
    ///   2. Contract transfers `amount` USDC from student → contract via the
    ///      USDC token's `transfer` function (SEP-41 / Stellar Asset Contract).
    ///   3. Vault record is stored with the unlock timestamp.
    pub fn deposit(env: Env, student: Address, amount: i128, unlock_timestamp: u64) {
        // Student must authorize the deposit
        student.require_auth();

        if amount <= 0 {
            panic!("deposit amount must be positive");
        }
        if unlock_timestamp <= env.ledger().timestamp() {
            panic!("unlock date must be in the future");
        }

        // If a vault already exists, add to it (same unlock date preserved)
        let key = DataKey::Vault(student.clone());
        let existing: Option<Vault> = env.storage().persistent().get(&key);

        let new_vault = match existing {
            Some(mut v) => {
                if !v.is_open {
                    panic!("vault is closed; open a new one by depositing again");
                }
                v.balance += amount;
                v
            }
            None => Vault {
                student: student.clone(),
                balance: amount,
                unlock_timestamp,
                is_open: true,
            },
        };

        // Pull USDC from the student's wallet into this contract
        let usdc: Address = env
            .storage()
            .instance()
            .get(&DataKey::UsdcToken)
            .unwrap_or_else(|| panic!("contract not initialized"));

        let usdc_client = token::Client::new(&env, &usdc);
        usdc_client.transfer(&student, &env.current_contract_address(), &amount);

        // Persist the updated vault
        env.storage().persistent().set(&key, &new_vault);
    }

    /// Student withdraws their full balance after the unlock date.
    ///
    /// Flow:
    ///   1. Student authorizes the call.
    ///   2. Contract checks ledger timestamp ≥ unlock_timestamp.
    ///   3. Contract transfers full balance back to student via USDC token contract.
    ///   4. Vault balance zeroed and marked closed.
    pub fn withdraw(env: Env, student: Address) -> i128 {
        student.require_auth();

        let key = DataKey::Vault(student.clone());
        let mut vault: Vault = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| panic!("no vault found for this address"));

        if !vault.is_open {
            panic!("vault is already closed");
        }
        if vault.balance == 0 {
            panic!("nothing to withdraw");
        }

        // Enforce the time lock
        let now = env.ledger().timestamp();
        if now < vault.unlock_timestamp {
            panic!("vault is still locked; try again after the unlock date");
        }

        let amount = vault.balance;

        // Transfer USDC from contract back to student
        let usdc: Address = env
            .storage()
            .instance()
            .get(&DataKey::UsdcToken)
            .unwrap_or_else(|| panic!("contract not initialized"));

        let usdc_client = token::Client::new(&env, &usdc);
        usdc_client.transfer(&env.current_contract_address(), &student, &amount);

        // Close and zero the vault
        vault.balance = 0;
        vault.is_open = false;
        env.storage().persistent().set(&key, &vault);

        amount
    }

    /// Read a student's current vault state.
    pub fn get_vault(env: Env, student: Address) -> Vault {
        env.storage()
            .persistent()
            .get(&DataKey::Vault(student))
            .unwrap_or_else(|| panic!("no vault found"))
    }

    /// How many seconds remain until a student's vault unlocks.
    /// Returns 0 if already unlocked.
    pub fn seconds_until_unlock(env: Env, student: Address) -> u64 {
        let vault: Vault = env
            .storage()
            .persistent()
            .get(&DataKey::Vault(student))
            .unwrap_or_else(|| panic!("no vault found"));

        let now = env.ledger().timestamp();
        if now >= vault.unlock_timestamp {
            0
        } else {
            vault.unlock_timestamp - now
        }
    }
}
