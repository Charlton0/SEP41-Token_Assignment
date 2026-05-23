# SEP-41 Token Smart Contract (Soroban)

This project is a Rust-based smart contract built using the **Soroban SDK** for Stellar blockchain.  
It implements a fungible token following the **SEP-41 Token Standard**, including core token functionalities such as minting, transferring, burning, and allowance-based transfers.

---

## 🚀 Features

The contract supports the following functionality:

### 🔹 Token Metadata
- Token name
- Token symbol
- Token decimals

### 🔹 Core Token Operations
- Mint new tokens (admin only)
- Transfer tokens between accounts
- Burn tokens from an account
- Check account balance
- Check total supply

### 🔹 Authorization & Security
- Admin-controlled minting
- `require_auth()` enforced on sensitive operations
- Unauthorized access prevention

### 🔹 Allowance System (ERC20-style)
- Approve spender allowance
- Transfer on behalf of another account (`transfer_from`)
- Allowance tracking per `(owner, spender)` pair

