# Pyra Instructions

Instruction builders for the Pyra protocol on Solana. This crate provides functions to construct Solana instructions and transactions for all Pyra program operations.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
pyra-instructions = "0.2"
```

### Modules

- **`user`** — User-facing instructions: account initialization, spend limit updates, and Drift setup
- **`admin`** — Admin instructions: spend settlement, refunds, and account closure
- **`drift`** — Drift protocol instructions: deposits, withdrawals, swaps, and order fulfillment
- **`operations`** — Composite operations that bundle multiple instructions into transactions

## License

This project is licensed under [MIT license](../LICENSE).
