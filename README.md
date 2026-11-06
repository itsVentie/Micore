# **Micore**

### Tech Stack

- **Backend:** Rust (Cargo Workspace), Tauri v2, `redb` (Custom `.mic` storage), `core-crypto` (Argon2id KDF + AES-256-GCM)
- **Frontend:** TypeScript, Svelte, Bun

---

<details>
<summary><b>Phase 1: Core Architecture & Storage (Foundation)</b></summary>

- [x] Set up Cargo Workspace structure (`core-crypto`, `core-storage`, `vault-app`, `ui`)
- [x] Implement secure storage layer with custom `.mic` file format (`redb` + Argon2id KDF + AES-256-GCM)
- [x] Set up IPC bridge commands between Rust core and Webview host
- [x] Implement zero-memory trace policies (secure memory wiping on lock/idle)

</details>

<details>
<summary><b>Phase 2: Core Modules & Dashboard UI</b></summary>

- [ ] **Contacts & Profiles:** Network graph, identities, cross-linked metadata
- [ ] **Bookmark Manager:** Link storage, OpenGraph title auto-fetch, tag systems
- [ ] **Code Snippets & Vault:** Syntax-highlighted snippets and one-click secrets copy
- [ ] **Knowledge Base:** Markdown editor with live preview and cross-linking

</details>

<details>
<summary><b>Phase 3: Search, Interactivity & Optimization</b></summary>

- [ ] Implement global fuzzy search palette (`Ctrl+K`)
- [ ] Interactive graph visualization for interconnected items
- [ ] Full keyboard-driven navigation (`vim`-style keybindings)
- [ ] Encrypted export/import pipeline (Backup to single `.mic` payload)

</details>

<details>
<summary><b>Phase 4: DevSecOps & Release Engineering</b></summary>

- [ ] Configure GitHub Actions CI/CD (Cross-compilation for Windows/Linux/macOS)
- [ ] Enforce security checks (`cargo-clippy`, `cargo-audit`, dependency pinning)
- [ ] Write documentation and cut release v1.0.0

</details>

---

### Getting Started

#### Prerequisites

- Bun (v1.0+)
- Rust (latest stable)

#### Installation

1. Clone the repository:

```bash
git clone https://github.com/itsventie/Micore.git
cd Micore

```

2. Install frontend dependencies:

```bash
cd ui
bun install
cd ..

```

3. Launch in development mode:

```bash
cargo tauri dev

```

