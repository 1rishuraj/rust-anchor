# Solana Learning Journey: Native Rust to Anchor

This repository documents my progression through Solana development, starting with native Rust programs and moving into higher-level Anchor workflows. Each folder is a focused mini-project exploring one concept deeply.

## [native-solana-counter](./native-solana-counter)
A foundational native counter project split between an on-chain Rust program and a separate TypeScript/Bun client test harness, focused on understanding Borsh serialization, account ownership, and transaction-driven state updates in a minimal counter workflow.

## [native-solana-calculator-contract](./native-solana-calculator-contract)
A native Solana Rust calculator-style program that stores a `u32` in account data and supports core operations like initialize, double, half, add, and subtract, with end-to-end local execution tested using Bun + LiteSVM to validate instruction encoding, state transitions, and account writes.

## [native-solana-cpi-for-data-acc](./native-solana-cpi-for-data-acc)
A native contract example that demonstrates creating a dedicated data account via CPI to the System Program’s `create_account`, emphasizing account allocation/funding mechanics and the pattern for provisioning program-owned storage during instruction execution.

## [native-solana-cpi-for-pda](./native-solana-cpi-for-pda)
A native Solana example centered on Program Derived Address creation, showing PDA derivation from deterministic seeds, verification of expected PDA inputs, and `invoke_signed` use to create and fund a program-owned PDA account through the System Program.

## [native-solana-cpi-to-contract](./native-solana-cpi-to-contract)
A native CPI learning project where one program (`cpi`) invokes another (`double`) to mutate shared state, demonstrating how cross-program invocation works in practice, how instruction/account plumbing is handled, and how repeated CPI calls affect persisted on-chain data.

## [anchor-calculator](./anchor-calculator)
An Anchor-based version of the calculator/counter idea that implements initialize, double, half, add, and subtract instructions using Anchor accounts and contexts, intended to contrast native boilerplate-heavy patterns with Anchor’s ergonomic account validation and instruction handling.

## [anchor-transferCPI-builtin](./anchor-transferCPI-builtin)
An Anchor CPI transfer example that uses Anchor’s built-in System Program transfer helpers (`transfer` + `CpiContext`) to move lamports between accounts, focusing on the simplest idiomatic Anchor approach for SOL transfers through CPI.

## [anchor-transferCPI-manually](./anchor-transferCPI-manually)
An Anchor CPI transfer counterpart that manually constructs and invokes the System Program transfer instruction to expose the lower-level mechanics behind CPI, making it easier to compare helper-driven vs manual-instruction approaches inside Anchor programs.

## [anchor-pda](./anchor-pda)
An Anchor PDA project that uses Anchor account constraints and PDA seeds/bump handling to create and manage program-derived accounts, helping bridge from manual native PDA creation toward idiomatic Anchor patterns for secure deterministic accounts.
