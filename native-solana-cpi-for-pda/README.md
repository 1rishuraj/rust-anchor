# Solana PDA Creation Example

This project demonstrates how to create a **Program Derived Address (PDA)** in Solana using a custom on-chain program and `invoke_signed` to CPI into the **System Program**.

---

## 📌 What This Program Does

- Derives a PDA using the seeds:
  - `b"client1"` (namespace)
  - `payer` public key
- Verifies that the correct PDA is passed by the client
- Creates a **program-owned PDA account**
- Allocates **4 bytes of storage** (for future `u32` data, e.g., a counter)
- Funds the PDA using the payer’s lamports


---
