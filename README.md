# StudySave DApp

**StudySave DApp** - Blockchain-Based Decentralized Savings Vault for Students

## Project Description

StudySave DApp is a decentralized smart contract solution built on the Stellar blockchain using Soroban SDK. It provides a secure, time-locked savings vault for university students in Southeast Asia who struggle to manage their weekly allowance. The contract ensures that deposited USDC is held transparently on-chain and can only be withdrawn after a student-defined unlock date, eliminating reliance on willpower or centralized banking apps.

The system allows students to deposit USDC, lock it until a chosen date, and withdraw it automatically when the time comes — leveraging the speed, low cost, and programmability of the Stellar network. Each vault is uniquely tied to the student's wallet address and stored in persistent contract storage, ensuring data reliability and tamper-proof enforcement.

## Project Vision

Our vision is to make financial discipline accessible to every student in Southeast Asia by:

- **Decentralizing Savings**: Moving personal savings from bank accounts and cash envelopes to a programmable, student-controlled blockchain vault
- **Ensuring Ownership**: Empowering students to have full custody of their funds without needing a bank account or credit history
- **Guaranteeing Enforcement**: Providing a time-lock mechanism that cannot be overridden — not by the app, not by an admin, only by the smart contract rules
- **Eliminating Volatility Risk**: Using USDC as the savings currency so students are not exposed to crypto price swings
- **Building Financial Habits**: Creating a platform where saving is automatic, transparent, and rewarding — not a chore

We envision a future where any student with a smartphone and a Stellar wallet can build financial discipline and a verifiable savings track record — without a bank.

## Key Features

### 1. **USDC-Based Savings Vault**

- Deposit any amount of USDC directly from your Stellar wallet
- Funds are held inside the smart contract — not in a bank, not with an admin
- USDC ensures savings are stable and not subject to crypto price volatility
- Compatible with Circle USDC on the Stellar network

### 2. **Programmable Time Lock**

- Student chooses their own unlock date at the time of deposit
- Smart contract enforces the lock — withdrawal before the date is rejected automatically
- No override, no admin bypass, no exceptions — the code is the rule
- `seconds_until_unlock` helper function powers countdown timers on the frontend

### 3. **Top-Up Deposits**

- Students can add more USDC to their vault at any time before the unlock date
- Each top-up accumulates on the existing balance within the same vault
- Supports real student behavior: depositing weekly allowance in small increments

### 4. **Instant Withdrawal After Unlock**

- Once the unlock timestamp is reached, student calls `withdraw()` and receives full balance
- USDC is transferred back to the student's wallet in a single Stellar transaction
- Vault is automatically closed and zeroed after withdrawal

### 5. **Stellar Network Integration**

- Leverages the high speed and near-zero cost of Stellar (< $0.001 per transaction)
- Built using the modern Soroban Smart Contract SDK and Stellar Asset Contract (SAC) standard
- Directly integrates with the USDC token contract via `token::Client` — real on-chain transfers, not just ledger entries
- Interoperable with Freighter, Lobstr, and XUMM wallets

## Contract Details

- **Network:** Stellar Testnet
- **Contract Name:** `study_save`
- **Language:** Rust (Soroban SDK v21)
- **USDC Token (Testnet):** `GBBD47IF6LWK7P7MDEVSCWR7DPUWV3NY3DTQEVFL4NAT4AQH3ZLLFLA5`
- **Core Functions:** `init()` · `deposit()` · `withdraw()` · `get_vault()` · `seconds_until_unlock()`

## Future Scope

### Short-Term Enhancements

1. **Savings Goals**: Allow students to set a named goal (e.g. "Tuition Fee") alongside the unlock date for motivation
2. **Partial Withdrawal**: Support withdrawing a portion of the balance after unlock while keeping the rest locked
3. **Vault History**: Store a log of all deposits and withdrawals per student address for transparency
4. **Frontend Countdown Timer**: A mobile-friendly web app showing exactly how many days and hours remain until unlock

### Medium-Term Development

5. **Sponsor Top-Ups**: Allow NGOs, alumni, or parents to add USDC to a student's vault as a scholarship match
   - Admin-authorized sponsor address
   - On-chain proof of disbursement visible to all parties
   - Notification to student when a sponsor deposit arrives
6. **Multi-Vault Support**: Allow one wallet to hold multiple named vaults with different unlock dates
7. **Interest via DeFi**: Route locked USDC through a Stellar-based lending protocol to earn yield during the lock period
8. **Recurring Deposits**: Automate weekly deposits using a scheduled off-chain trigger connected to the contract

### Long-Term Vision

9. **Cross-Border Savings**: Accept remittances from OFW parents directly into a child's StudySave vault
10. **AI-Powered Savings Coach**: Claude-powered companion that analyses deposit patterns and sends personalized nudges via Telegram or SMS — e.g. *"You've saved ₱800 so far this week. Deposit ₱200 more today to hit your Friday goal."*
11. **On-Chain Credit Score**: Use vault deposit history and on-time unlock behavior as input to a student credit profile for future lending products
12. **DAO Governance**: Community-driven interest rate and feature decisions governed by a student cooperative token
13. **Decentralized UI Hosting**: Host the frontend on IPFS for censorship-resistant access across SEA
14. **Identity Integration**: Link vault activity to a decentralized student identity (DID) for verifiable financial history

### Enterprise & Institutional Features

15. **University Integration**: Allow universities to verify on-chain that a student has saved enough for tuition before confirming enrollment
16. **NGO Disbursement Tracking**: Provide scholarship organizations with a transparent view of how their top-ups are being saved and used
17. **Group Savings Pools**: Extend to paluwagan-style rotating savings groups for student cooperatives
18. **Multi-Language Support**: Tagalog, Bahasa Indonesia, and Vietnamese UI for broader SEA accessibility

---

## Technical Requirements

- Rust (stable) — https://rustup.rs
- Soroban SDK v21
- Soroban CLI v21+
- Stellar blockchain network (Testnet for development, Mainnet for production)

## Getting Started

Deploy the smart contract to Stellar's Soroban network and interact with it using the five main functions:

- `init()` — Initialize the contract with the USDC token address (called once on deploy)
- `deposit()` — Lock USDC into the vault with a chosen unlock timestamp
- `withdraw()` — Retrieve full balance after the unlock date has passed
- `get_vault()` — View current vault state: balance, unlock date, open/closed status
- `seconds_until_unlock()` — Check how many seconds remain until the vault unlocks

### Build

```bash
soroban contract build
```

### Test

```bash
cargo test
```

### Deploy to Testnet

```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/study_save.wasm \
  --source student \
  --network testnet
```

### Sample CLI Invocation — Deposit 20 USDC, unlock in 7 days

```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source student \
  --network testnet \
  -- deposit \
  --student $(soroban keys address student) \
  --amount 200000000 \
  --unlock_timestamp $(( $(date +%s) + 604800 ))
```

> `200000000` = 20 USDC (USDC uses 7 decimal places on Stellar)

---

**StudySave DApp** - Securing Your Allowance on the Blockchain

## Deployed Contract

| Field | Value |
|-------|-------|
| Contract ID | `CA22MV5ZXXKXKJWW5W3ICTOZHGC6OLMTSLNC4DRMLXGXHL25KKIVKM7L` |
| Network | testnet |
| Explorer | [View on stellar.expert](https://stellar.expert/explorer/testnet/contract/CA22MV5ZXXKXKJWW5W3ICTOZHGC6OLMTSLNC4DRMLXGXHL25KKIVKM7L) |
| Deploy Tx | [View transaction](https://stellar.expert/explorer/testnet/tx/ff9902f29dd33f69b07c8493c2a0865ff2cf763aa3f8108bf1ab5f926f60055f) |
| Deployed | 2026-06-26 06:57:11 UTC |
| Wallet | freighter (`GAYD…DTXN`) |
