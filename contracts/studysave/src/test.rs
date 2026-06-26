#[cfg(test)]
mod tests {
    use soroban_sdk::{
        testutils::{Address as _, Ledger},
        token::{Client as TokenClient, StellarAssetClient},
        Address, Env,
    };
    use crate::{StudySave, StudySaveClient};

    // ─── Test Helpers ──────────────────────────────────────────────────────────

    /// Deploy a mock USDC token contract and mint `amount` to `recipient`.
    fn create_usdc_token(env: &Env, admin: &Address) -> Address {
        let token_id = env.register_stellar_asset_contract_v2(admin.clone())
            .address();
        token_id
    }

    fn mint_usdc(env: &Env, token: &Address, admin: &Address, to: &Address, amount: i128) {
        let sac = StellarAssetClient::new(env, token);
        sac.mint(to, &amount);
    }

    /// Deploy StudySave, init with mock USDC, return (client, usdc_address, student).
    fn setup() -> (Env, StudySaveClient<'static>, Address, Address, Address) {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let student = Address::generate(&env);

        let usdc = create_usdc_token(&env, &admin);
        // Give student 1000 USDC (in 7-decimal stroops: 1 USDC = 10_000_000)
        mint_usdc(&env, &usdc, &admin, &student, 1_000_000_0000_i128);

        let contract_id = env.register_contract(None, StudySave);
        let client = StudySaveClient::new(&env, &contract_id);
        client.init(&usdc);

        (env, client, usdc, admin, student)
    }

    // ─── Test 1: Happy path ───────────────────────────────────────────────────
    // Student deposits 20 USDC, waits until after unlock, withdraws full amount.
    #[test]
    fn test_deposit_and_withdraw_after_unlock() {
        let (env, client, usdc, _admin, student) = setup();

        // Set ledger to timestamp 1000
        env.ledger().with_mut(|l| l.timestamp = 1_000);

        // Deposit 20 USDC; unlock in 7 days (604800 seconds)
        let amount = 200_000_000_i128; // 20 USDC at 7 decimals
        client.deposit(&student, &amount, &1_605_800_u64); // unlock at ts 1_605_800

        let vault = client.get_vault(&student);
        assert_eq!(vault.balance, amount);
        assert!(vault.is_open);

        // Fast-forward past unlock date
        env.ledger().with_mut(|l| l.timestamp = 2_000_000);

        let withdrawn = client.withdraw(&student);
        assert_eq!(withdrawn, amount);

        // Vault should be closed and zeroed
        let vault_after = client.get_vault(&student);
        assert_eq!(vault_after.balance, 0);
        assert!(!vault_after.is_open);
    }

    // ─── Test 2: Edge case ────────────────────────────────────────────────────
    // Withdrawal before unlock date must panic.
    #[test]
    #[should_panic(expected = "vault is still locked")]
    fn test_withdraw_before_unlock_panics() {
        let (env, client, _usdc, _admin, student) = setup();

        env.ledger().with_mut(|l| l.timestamp = 1_000);
        client.deposit(&student, &200_000_000_i128, &9_999_999_u64);

        // Try to withdraw while still locked
        env.ledger().with_mut(|l| l.timestamp = 2_000); // still before 9_999_999
        client.withdraw(&student); // must panic
    }

    // ─── Test 3: State verification ───────────────────────────────────────────
    // Multiple top-up deposits accumulate correctly in the same vault.
    #[test]
    fn test_multiple_deposits_accumulate() {
        let (env, client, _usdc, _admin, student) = setup();

        env.ledger().with_mut(|l| l.timestamp = 1_000);

        // Student deposits three times in the same week
        client.deposit(&student, &50_000_000_i128, &999_999_u64); // 5 USDC
        client.deposit(&student, &70_000_000_i128, &999_999_u64); // 7 USDC
        client.deposit(&student, &80_000_000_i128, &999_999_u64); // 8 USDC

        let vault = client.get_vault(&student);
        // Total: 20 USDC locked
        assert_eq!(vault.balance, 200_000_000_i128);
        assert!(vault.is_open);
    }

    // ─── Test 4: Edge case ────────────────────────────────────────────────────
    // Depositing with an unlock date in the past must panic.
    #[test]
    #[should_panic(expected = "unlock date must be in the future")]
    fn test_deposit_with_past_unlock_panics() {
        let (env, client, _usdc, _admin, student) = setup();

        env.ledger().with_mut(|l| l.timestamp = 5_000);
        // unlock_timestamp of 1_000 is already in the past at ts 5_000
        client.deposit(&student, &100_000_000_i128, &1_000_u64);
    }

    // ─── Test 5: seconds_until_unlock returns 0 once unlocked ─────────────────
    #[test]
    fn test_seconds_until_unlock_counts_down_to_zero() {
        let (env, client, _usdc, _admin, student) = setup();

        env.ledger().with_mut(|l| l.timestamp = 1_000);
        client.deposit(&student, &100_000_000_i128, &2_000_u64);

        // Before unlock: should return 1000 seconds remaining
        let remaining = client.seconds_until_unlock(&student);
        assert_eq!(remaining, 1_000_u64);

        // After unlock: should return 0
        env.ledger().with_mut(|l| l.timestamp = 3_000);
        let remaining_after = client.seconds_until_unlock(&student);
        assert_eq!(remaining_after, 0_u64);
    }
}
