# Freezeable-Token-Contract
This project extends the standard Stellar token contract by adding freeze and unfreeze functionalities. 

## Description
This contract is based on the Stellar token standard and introduces two new functions: `freeze_account` and `unfreeze_account`. When an account is frozen, it cannot transfer tokens until it is unfrozen. Only the account owner can freeze or unfreeze their own account. The `transfer` function is modified to block transfers from frozen accounts.

## How to Use
- Deploy the contract to the Stellar testnet.
- Use the `freeze_account` function to freeze your tokens.
- Use the `unfreeze_account` function to unlock your tokens.
- Attempting to transfer tokens from a frozen account will fail.

## Contract Address

