# Pyra Tokens

A Rust crate containing supported tokens for Pyra Solana smart contracts. This crate provides a shared interface for token metadata that can be used by both on-chain programs and backend services.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
pyra-tokens = "0.2"
```

### Basic Usage

```rust
use pyra_tokens::{Token, SUPPORTED_TOKENS};

// Access all supported tokens
for token in SUPPORTED_TOKENS.iter() {
    println!("Token at market index {}: {}", token.market_index, token.mint);
}

// Find token by market index
if let Some(token) = Token::find_by_market_index(0) {
    println!("USDC mint: {}", token.mint);
}

// Find token by mint address
use solana_program::pubkey;
let usdc_mint = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
if let Some(token) = Token::find_by_mint(&usdc_mint) {
    println!("USDC market index: {}", token.market_index);
}
```

## License

This project is licensed under [MIT license](../LICENSE).
