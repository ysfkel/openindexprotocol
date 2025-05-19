 # Open Index Protocol

*Solana‑native, modular index‑fund engine*

[![License: MIT/Apache](https://img.shields.io/badge/license-MIT%20%2F%20Apache--2.0-blue)](LICENSE)  [![Build](https://img.shields.io/github/actions/workflow/status/OpenIndexProtocol/open-index-protocol/ci.yml?label=tests)](https://github.com/OpenIndexProtocol/open-index-protocol/actions)

---

## ✨ What is it?

Open Index Protocol lets anyone create, mint, redeem, and manage **tokenized index funds** on Solana. Think **Index Coop + Set Protocol**, but rebuilt for Solana‑level speed, fees, and composability.

* **Phase 1 (live)** – Core program: create indexes, vault custody, permissionless mint/redeem.
* **Phase 2 (grant milestones)** – Plug‑in Rebalancer + Trade Module, oracle feeds, governance configs, SDK & UI.

---

## 🔧 Quick Start

```bash
git clone https://github.com/OpenIndexProtocol/open-index-protocol.git
cd open-index-protocol
cargo build-bpf          # compile on‑chain program
cargo test-bpf           # run unit + integration tests
```

> **Prerequisites:** Rust `stable`, Solana CLI ≥1.17, Anchor ≥0.29 for local testing.

---

## 🗂 Repo Layout

```
programs/openindex/    # Phase‑1 core on‑chain program
programs/trade-module/ # Phase‑2 DEX CPI router (WIP)
programs/rebalancer/   # Phase‑2 rebalance logic  (WIP)
js/                    # TypeScript client SDK (WIP)
docs/                  # Markdown specs & diagrams
```

---

## ⚙️ High‑Level Architecture

```
┌────────────┐         attach via init_module          ┌───────────────────┐
│  Protocol  │ ───────────────────────────────────────▶│  Trade Module     │
└────┬───────┘                                        └─────────┬─────────┘
     │ create controller                                        │ swaps via
┌────▼───────┐ 1:N  mint/redeem                                 │ CPI to DEX
│ Controller │──────────────────────┐                           │
└────┬───────┘                      │ CPI fetch                 │
     │ create index                 │                           │
┌────▼───────┐            ┌───────────────────┐   CPI trigger   │
│   Index    │──holds───▶ │   Vault PDAs      │<────────────────┘
└────────────┘            └───────────────────┘
         ▲   ▲
         │   │ add components
         │   └──────────┐
┌────────┴──────────┐   │
│ Rebalancer Module │◀──┘ (computes drift, calls Trade Module)
└───────────────────┘
```

For a deep dive on accounts, PDAs, and execution flow, check the full docs below.

---

## 📚 Full Technical Docs

* **GitBook:** [https://openindex.gitbook.io/docs](https://openindex.gitbook.io/docs)
* **Grant proposal PDF:** [`/docs/Grant_Proposal.pdf`](./docs/Grant_Proposal.pdf)

---

## 🛠 Development Roadmap

| Status | Feature              | Notes                                |
| ------ | -------------------- | ------------------------------------ |
| ✅      | Phase‑1 core program | Mainnet‑beta ready, audited in‑house |
| 🛠     | Rebalancer Module    | Oracle integration + drift math      |
| 🛠     | Trade Module         | Jupiter/Phoenix router via CPI       |
| 🛠     | Governance config    | Controller‑level policies            |
| 🛠     | SDK & React demo     | Devnet front‑end + TS client         |

Track progress in the [milestone board](https://github.com/OpenIndexProtocol/open-index-protocol/projects/1).

---

## 🤝 Contributing

Pull requests are welcome! Please open an issue first if you plan a large change. Run `cargo fmt && cargo clippy` before submitting.

---

## 🪪 License

Dual‑licensed under **MIT** and **Apache 2.0**. Choose whichever license best fits your needs.
