# Stellar Soroban Token Contract – Freeze & Unfreeze Functionality

## Overview

This project extends a basic Stellar Soroban token contract by adding two new features: the ability for a token holder to freeze and later unfreeze their own tokens. When an account is frozen, it cannot transfer tokens until it is unfrozen.

## Features

- **freeze_account(account: Address):**  
  Allows a token holder to freeze their own account. Once frozen, no transfers can be made from that account.

- **unfreeze_account(account: Address):**  
  Allows a token holder to unfreeze their own account and resume transfers.

- **transfer(from: Address, to: Address, amount: i128):**  
  Updated to check if the sender's account is frozen. If so, the transfer is rejected.

## How It Works

- Each account can freeze or unfreeze itself by calling the appropriate function.
- The contract maintains a mapping of frozen accounts in storage.
- The transfer function checks this mapping and blocks transfers from frozen accounts.

## Deployment

- Deploy the contract to the Stellar testnet.
- Add your contract address here:  
  **Contract Address:** `<YOUR_CONTRACT_ADDRESS>`

## Usage Example

1. Call `freeze_account` with your own address to freeze your tokens.
2. Attempting to transfer tokens while frozen will fail.
3. Call `unfreeze_account` with your own address to unfreeze and enable transfers again.

## New Functionality

- Self-service freezing and unfreezing of tokens for security or compliance needs.
- Transfer function now enforces the freeze status.

---

**Developed for the Stellar Soroban Bootcamp Final Project.**
