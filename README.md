# fedimint-tauri

> ⚠️ **Warning: This project is under active development. Features may be incomplete or unstable. Use at your own risk.**

A Tauri-based SDK and example app to build privacy-focused wallets with **e-cash** support using [Fedimint](https://fedimint.org). Built for developers who want a fast, secure, and modular foundation for creating wallets that respect user privacy.

---

## 🚀 Features Timeline

Below is a versioned timeline of feature development. Checkboxes show what’s done and what’s coming.

### ✅ `v0.0.1-alpha`
- [x] Project init
- [ ] Setup Development Environment (nix-shell/flake)
- [ ] Fedimint client integration
- [ ] Basic e-cash wallet
- [ ] Example app setup

---

### 🔄 `v0.0.2-alpha` - **General**
- [ ] Basic functions.
- [ ] Android Setup and initial testing.
- [ ] 🧾 Pay Lightning invoice
- [ ] 📤 Export e-cash  
- [ ] 📥 Import e-cash  
- [ ] 📫 Lightning Address support  
- [ ] 💰 Receive tokens by paying a Lightning invoice  

---

### 🔄 `v0.0.3-alpha` - **Federations**
- [ ] 🛡️ Show guardian health  
- [ ] ♻️ Restore wallet  
- [ ] 🗃️ Backup wallet  
- [ ] 👤 Pay to Nostr contacts  

---

### 🔜 `v0.1.0-beta` and beyond
- [ ] 🧭 Discover federations using Nostr  
- [ ] 📊 Show Nostr votes for federation
- [ ] 🎨 UI polish & UX flows  
- [ ] 📦 Cross-platform packaging  
- [ ] 🧪 Testing + CI/CD pipeline  
- [ ] 📚 Full developer documentation  

---

## 🛠️ Getting Started

```bash
git clone https://github.com/fedimint/fedimint-tauri.git
cd fedimint-tauri
cargo tauri dev