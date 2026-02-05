# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is the **Pyra Rust SDK** repository, containing Rust crates for interacting with the Pyra DeFi protocol on Solana. The SDK is designed to be split across multiple Rust crates to provide modular functionality for on-chain programs and backend services.

## Repository Structure

### Current Crates

- **`tokens/`** - The `pyra-tokens` crate containing supported token definitions
  - Published crate with comprehensive token metadata
  - Includes 10 supported tokens (USDC, USDT, SOL, etc.) with market indices and mint addresses
  - Provides lookup functions by market index or mint address
  - Supports both SPL Token and SPL Token-2022 programs

- **`token-definitions/`** - The `pyra-token-definitions` crate (work in progress)
  - New crate structure created but not yet finalized
  - Intended to complement or replace the tokens crate

### Key Architecture Concepts

**Token Management**: The core concept is the `Token` struct which combines:
- `market_index: u16` - Unique identifier within the Pyra ecosystem
- `mint: Pubkey` - Solana mint address of the token
- `token_program: Pubkey` - Either SPL Token or SPL Token-2022 program ID

**Static Configuration**: All supported tokens are defined in a static array `SUPPORTED_TOKENS` for compile-time safety and on-chain compatibility.

## Development Commands

This repository uses standard Cargo commands as there are no custom build scripts:

```bash
# Check compilation (run after each task)
cargo check

# Build all crates
cargo build

# Build specific crate
cd tokens/ && cargo build

# Run tests
cargo test

# Generate documentation
cargo doc --open

# Format code
cargo fmt

# Run linter
cargo clippy
```

**Important**: Always run `cargo check` after completing each development task to ensure the code compiles correctly.

## Publishing Workflow

For publishing crates to crates.io:

```bash
# Dry run to test publishing
cargo publish --dry-run

# Actually publish
cargo publish

# Login to crates.io (one-time setup)
cargo login
```

## Dependencies

Core dependencies used across crates:
- **solana-program** (v2.1) - Core Solana blockchain functionality
- **spl-token** (v6.0) - Standard Program Library for tokens
- **spl-token-2022** (v4.0) - Extended token standard support

## Integration Context

This Rust SDK is part of a larger Pyra ecosystem that includes:
- Solana smart contracts (`pyra-protocol/`)
- TypeScript SDK (`sdk/`)
- Web and mobile applications
- Backend services and infrastructure

The Rust SDK serves as a bridge between on-chain programs and off-chain services, providing shared type definitions and utilities.

## Crate Metadata Standards

When creating new crates, follow this pattern:
- **License**: "MIT OR Apache-2.0"
- **Repository**: "https://github.com/pyra-labs/pyra-rust-sdk"
- **Keywords**: ["solana", "token", "defi", "blockchain"]
- **Categories**: ["api-bindings", "cryptography::cryptocurrencies"]
- **Author**: "Iarla Crewe <iarla@pyra.fi>"

## Code Style Guidelines

**Documentation Philosophy**: Keep comments and documentation concise. Code should be self-explanatory through clear naming and structure. Add sharp, focused documentation only where it provides genuine value - typically for public APIs, complex algorithms, or non-obvious design decisions.

## Important Notes

- The repository currently has no workspace configuration - each crate is independent
- No CI/CD or automated testing infrastructure is currently set up
- Documentation examples should use the actual Pyra token mint addresses from the SUPPORTED_TOKENS array
- Public APIs should have concise documentation with examples where helpful
- Token definitions must support both on-chain (program) and off-chain (service) usage patterns