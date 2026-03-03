# Anchor Calculator

A simple Solana smart contract built with **Anchor** that demonstrates creating and updating on-chain data accounts.

## 📘 Overview

This program creates a data account (`NewAccount`) to store a single `u32` value, then allows you to:
- **Initialize** it with a starting count of `1`
- **Double** the count
- **Halve** the count
- **Add** or **Subtract** custom amounts

---

## ⚙️ Setup

### 1️⃣ Prerequisites
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli)
- [Anchor CLI](https://www.anchor-lang.com/)
- [Node.js](https://nodejs.org/)
- Local validator (Anchor will start this automatically)

### 2️⃣ Install Dependencies
```bash
npm install
````

### 3️⃣ Build the Program

```bash
anchor build
```

### 4️⃣ Run Tests

```bash
anchor test
```

This command:

* Starts a local Solana test validator
* Deploys your program
* Runs Mocha tests in `tests/test.ts`

---

## 🛠️ Folder Structure

```
├── programs/
│   └── calc/   # Rust on-chain program
├── tests/
│   └── test.ts # Mocha tests using Anchor JS
├── target/
│   └── idl/                 # Generated IDL & TypeScript types
└── Anchor.toml              # Anchor configuration
```
