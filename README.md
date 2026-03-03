# 🧪 Solana CPI Test – Double via CPI

This project demonstrates how to **perform a Cross-Program Invocation (CPI)** on Solana using `LiteSVM` and two smart contracts (`cpi.so` and `double.so`) entirely in a local testing environment.

---

## 📂 Project Structure

```
/project-root
│
├── folder/
│   ├── cpi.so           # Program that invokes double program via CPI
│   └── double.so        # Program that doubles a stored u32 counter
│
├── double.test.ts       # Bun test invoking CPI calls 4 times
└── README.md            # You are here
```

---

## ⚙️ Requirements

* **Node.js / Bun**
  Install Bun for faster tests:

  ```bash
  curl -fsSL https://bun.sh/install | bash
  ```

* **Rust + Solana SDK**
  To build `.so` binaries:

  ```bash
  cargo build-bpf --manifest-path Cargo.toml --bpf-out-dir=folder/
  ```

---

## 🧠 Program Overview

### 🔹 `double.so`

A simple Solana program that stores and doubles a counter.

**Logic:**

```rust
if count == 0:
    count = 1
else:
    count *= 2
```

Each call doubles the stored value.
It uses **Borsh serialization** to manage the 4-byte counter in account data.

---

### 🔹 `cpi.so`

A program that **invokes** the `double.so` program via a CPI.
