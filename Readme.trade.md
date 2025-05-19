
## 📦 `Trade Module` 

````markdown
# Trade Module for Open Index Protocol

This is a modular DEX execution engine designed to be registered with [Open Index Protocol](https://github.com/openindex-protocol/open-index-core) via the `init_module` interface.

It routes token swaps through integrated Solana DEXs (e.g., Jupiter, Phoenix) and returns output to index vaults as part of rebalancing or strategy execution.

---

## 🔧 Features

- 🔁 Executes token swaps via CPI to Jupiter or Phoenix
- 🔌 Registered dynamically using `init_module` instruction
- 🔐 Isolated logic from core protocol (security by separation)
- 🧩 Callable by authorized modules like the Rebalancer

---

## 📦 Architecture

- Accepts instruction CPI from Rebalancer or authorized bots
- Executes trades via integrated DEX router
- Reports execution output to Open Index vaults

---

## 🧪 Test Environment

```bash
cargo test-bpf
````

---

## 🧠 Design Notes

* Built with future-proof modularity
* Reusable by other vault or fund protocols
* Follows SPL-style permission model

---

## 🪪 License

MIT or Apache 2.0

````
