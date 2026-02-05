# Anchor Vault Program

A simple Solana vault built with Anchor that lets a user initialize a vault, deposit SOL, withdraw SOL, and close the vault.

## Features

* Initialize vault
* Deposit SOL
* Withdraw SOL
* Close vault and reclaim funds
* Test cases for each instructions

## Stack

* Solana
* Anchor
* TypeScript tests

## Run

```bash
anchor build
anchor test
```

## Run on local validator

```bash
anchor build
surfpool start (in separate terminal)
anchor test --skip-local-validator
```
---

## Screenshot of Test cases passed
<img width="3736" height="2242" alt="image" src="https://github.com/user-attachments/assets/87e6a03b-442e-42a6-87ac-2a1c50ab196c" />

