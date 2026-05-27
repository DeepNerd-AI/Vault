<div align="center">

<img src="https://capsule-render.vercel.app/api?type=waving&color=0:0d0d0d,40:1a0533,100:2d0a5e&height=220&section=header&text=Vault%20IDE&fontSize=72&fontColor=ffffff&fontAlignY=38&desc=A%20GPU-Accelerated%20Multiplayer%20Code%20Editor%20by%20DeepNerd-AI&descAlignY=60&descSize=16&animation=twinkling" width="100%" />

<br/>

[![Vault IDE Icon](https://github.com/DeepNerd-AI/Vault/raw/main/crates/zed/resources/app-icon.png)](https://github.com/DeepNerd-AI/Vault)

<br/>

![Built with Rust](https://img.shields.io/badge/Built%20With-Rust%20🦀-CE412B?style=flat-square&logo=rust&logoColor=white)
![Platform](https://img.shields.io/badge/Platform-Linux%20(+%20macOS%2FWin%20from%20source)-informational?style=flat-square&logo=linux&logoColor=white)
![License](https://img.shields.io/badge/License-AGPL%20%7C%20Apache%20%7C%20GPL--3.0-6D28D9?style=flat-square)
![Stars](https://img.shields.io/github/stars/DeepNerd-AI/Vault?style=flat-square&color=f59e0b)
![Issues](https://img.shields.io/github/issues/DeepNerd-AI/Vault?style=flat-square&color=ef4444)
![Commits](https://img.shields.io/github/commit-activity/m/DeepNerd-AI/Vault?style=flat-square&color=22c55e)
![Forks](https://img.shields.io/github/forks/DeepNerd-AI/Vault?style=flat-square&color=3b82f6)

<br/>

*A high-performance, multiplayer code editor — blazing fast, GPU-rendered, and built for the modern developer.*

</div>

---

## What is Vault?

**Vault** is a next-generation, GPU-accelerated code editor developed by [DeepNerd-AI](https://github.com/DeepNerd-AI). Built entirely in **Rust** and powered by a custom **GPUI** (GPU UI framework), Vault delivers an incredibly fluid, responsive editing experience with sub-8ms frame rendering targets.

Vault is a custom, heavily customized fork of the [Zed Code Editor](https://zed.dev), rebranded and extended under the DeepNerd-AI ecosystem, with its own identity, roadmap, and build infrastructure.

> **"Where every keystroke is instant, every collaboration is seamless, and your workspace is your vault."**

---

## ✨ Key Features

| Feature | Description |
|---------|-------------|
| ⚡ **GPU-Accelerated Rendering** | Powered by GPUI + WGPU — targets 120fps (≤8ms per frame) |
| 👥 **Multiplayer Collaboration** | Real-time co-editing via LiveKit integration |
| 🦀 **Pure Rust** | 97.9% Rust codebase — memory-safe, blazing fast, zero GC pauses |
| 🤖 **AI-Powered Editing** | First-class support for Anthropic, OpenAI, Google AI, DeepSeek, Mistral, Ollama, LM Studio & more |
| 🧩 **Extension System** | WASM-based extensions with a full extension API |
| 🔌 **LSP Support** | Deep language server protocol integration for intelligent completions |
| 🐞 **Integrated Debugger** | Built-in DAP (Debug Adapter Protocol) support |
| 📟 **Integrated Terminal** | Full terminal emulator via Alacritty backend |
| 🔍 **Fuzzy Finder** | Lightning-fast file/symbol search with nucleo |
| 🌿 **Git Integration** | Built-in git graph, blame, diff, and hosting provider support |
| 🎨 **Theme Engine** | Full theme system with dark/light modes and importable themes |
| 📓 **Notebook Support** | Jupyter notebook REPL integration |
| 🔒 **Rules & Agents** | `.rules` file system and AI agent support (Claude, Gemini, etc.) |
| 🌐 **Web Search** | Built-in web search provider integration |
| 🖥️ **Remote Development** | Remote server and connection support |
| 🐳 **Docker & Nix** | Full Docker, Docker Compose, and Nix flake support |

---

## 🧠 AI Integrations

Vault has **first-class AI support** baked into its codebase. The following AI providers and services are included as workspace crates:

<div align="center">

| Provider | Crate |
|----------|-------|
| 🟠 Anthropic (Claude) | `crates/anthropic` |
| 🟢 OpenAI | `crates/open_ai` |
| 🔵 Google AI (Gemini) | `crates/google_ai` |
| ⬛ DeepSeek | `crates/deepseek` |
| 🟣 Mistral / Codestral | `crates/mistral`, `crates/codestral` |
| 🟡 Ollama (local LLMs) | `crates/ollama` |
| 🔷 LM Studio | `crates/lmstudio` |
| ☁️ AWS Bedrock | `crates/bedrock` |
| 🔗 OpenRouter | `crates/open_router` |
| ⚡ GitHub Copilot | `crates/copilot` |
| 🛠️ Agent System | `crates/agent`, `crates/agent_ui` |
| 📋 Context Servers (MCP) | `crates/context_server` |

</div>

AI-assisted editing is managed through a unified `language_model` abstraction, making it easy to switch between providers or run multiple in parallel.

---

## 🏗️ Architecture Overview

Vault is a **Cargo workspace** with 200+ crates organized into clear layers:

```
Vault/
├── crates/
│   ├── gpui/           ← GPU UI framework (the heart of Vault)
│   ├── editor/         ← Core editor type & buffer management
│   ├── zed/            ← Main entry point & app wiring
│   ├── workspace/      ← Project grouping & state serialization
│   ├── project/        ← File tree & navigation
│   ├── lsp/            ← Language Server Protocol client
│   ├── language/       ← Language understanding & Tree-sitter
│   ├── theme/          ← Theme system & default themes
│   ├── ui/             ← Shared UI component library
│   ├── agent/          ← AI agent orchestration
│   ├── collab/         ← Real-time collaboration backend
│   ├── terminal/       ← Integrated terminal (Alacritty)
│   ├── dap/            ← Debug Adapter Protocol
│   ├── repl/           ← Jupyter REPL integration
│   └── ...200+ more
├── extensions/         ← Built-in extensions (HTML, GLSL, Proto, etc.)
├── tooling/            ← Build compliance & perf tooling
├── docs/               ← Development documentation
├── ci/                 ← CI/CD pipeline configs
├── nix/                ← Nix flake support
└── script/             ← Build & license scripts
```

**Rendering pipeline:** GPUI → WGPU (WebGPU) → Metal (macOS) / Vulkan/OpenGL (Linux) / DirectX (Windows)

---

## 📦 Installation

### Linux (Debian / Ubuntu / Pop!_OS) — Pre-built Package

Download the latest `.deb` from the [**Releases page**](https://github.com/DeepNerd-AI/Vault/releases):

```bash
sudo dpkg -i vault-linux-*.deb
sudo apt-get install -f   # fix any missing dependencies
```

> **Note:** Pre-built packages are currently **Linux only** due to CI/CD resource constraints. macOS and Windows users must build from source. We're actively working to expand our build infrastructure!

---

### Building from Source

**Prerequisites:** Rust (latest stable via `rustup`), system graphics libraries

```bash
# 1. Clone the repository
git clone https://github.com/DeepNerd-AI/Vault.git
cd Vault

# 2. Build in release mode
cargo build --release

# 3. Run Vault
./target/release/vault
```

**Platform-specific build guides:**

- 🐧 [Building for Linux](docs/src/development/linux.md)
- 🍎 [Building for macOS](docs/src/development/macos.md)
- 🪟 [Building for Windows](docs/src/development/windows.md)

---

### Nix

```bash
nix develop    # enter the dev shell
nix build      # build Vault via flake
```

---

## 🛠️ Development

### Quick Start

```bash
git clone https://github.com/DeepNerd-AI/Vault.git
cd Vault
cargo build                    # debug build
cargo run                      # run in debug mode
cargo build --release          # production build
```

### Key Crates to Know

| Crate | What it does |
|-------|-------------|
| `gpui` | GPU UI framework — all rendering, layout, events |
| `editor` | The core `Editor` type — text, cursors, selections |
| `zed` | Main entry point — all crates assembled here |
| `workspace` | Manages projects, pane layout, state persistence |
| `project` | File tree, search, language detection |
| `lsp` | LSP server communication |
| `language` | Tree-sitter grammars, syntax highlighting |
| `collab` | Real-time collaboration via LiveKit |
| `agent` | AI agent thread and tool calling |
| `theme` | Theme loading, color resolution |
| `cli` | `vault` CLI binary |

### Running Tests

```bash
cargo test                           # all tests
cargo test -p editor                 # specific crate
cargo test -p gpui                   # UI framework tests
```

### Linting

```bash
./script/clippy                      # full clippy pass
cargo fmt --all                      # format code
```

---

## 🤝 Contributing

We welcome contributions! Before you start, please read our full **[CONTRIBUTING.md](CONTRIBUTING.md)**.

### Quick Contribution Guidelines

✅ **We love:**
- Bug fixes and doc improvements
- Small enhancements that work on more platforms
- Extra keybindings or editor actions
- Tests and visual test updates

❌ **We typically don't merge:**
- Features better served as extensions (new languages, themes)
- Giant refactors without prior discussion
- Non-trivial changes with no tests
- AI-generated code the author doesn't fully understand
- Pure style changes that don't affect logic

### PR Checklist

Before submitting, make sure your PR:

- [ ] Clearly describes **what** you changed and **why**
- [ ] Includes tests (unit or visual)
- [ ] Works on **Linux** (macOS/Windows cross-compile is a bonus)
- [ ] Meets the **120fps / 8ms** frame budget for any UI changes
- [ ] Passes `cargo clippy` and `cargo fmt`
- [ ] Attaches screenshots/recordings for UI changes
- [ ] Is focused on **one thing only**

### Code of Conduct

All contributors must follow our [Code of Conduct](CODE_OF_CONDUCT.md). See also: [deepnerd.tech/code-of-conduct](https://deepnerd.tech/code-of-conduct)

---

## 🧩 Extension Development

Vault supports WASM-based extensions. You can build language extensions, themes, and tools using the `extension_api` crate.

```bash
# Build an extension
cargo build -p my-extension --release
```

Extensions that ship with Vault out of the box: `glsl`, `html`, `proto`, `test-extension`.

---

## 📄 Licensing

Vault uses a multi-license structure:

| License | Applies To |
|---------|-----------|
| [AGPL](LICENSE-AGPL) | Core editor (server-side usage) |
| [Apache 2.0](LICENSE-APACHE) | Client-side libraries and crates |
| [GPL 3.0](LICENSE-GPL) | Specific components |

License compliance is enforced via `cargo-about`. If CI fails on license checks, see [CONTRIBUTING.md#licensing](CONTRIBUTING.md#licensing) for resolution steps.

---

## 💖 Sponsorship & Support

Vault is developed by **[DeepNerd-AI](https://github.com/DeepNerd-AI)**. Sponsorships go directly toward:

- Expanding build infrastructure for **macOS and Windows** pre-built packages
- Improving CI/CD pipeline capacity
- Growing the DeepNerd ecosystem

[![Sponsor DeepNerd-AI](https://img.shields.io/badge/Sponsor-DeepNerd--AI-%23EA4AAA?style=for-the-badge&logo=github-sponsors)](https://github.com/sponsors/DeepNerd-AI)

---

## 🔗 Links

<div align="center">

[![DeepNerd](https://img.shields.io/badge/DeepNerd-deepnerd.tech-7C3AED?style=for-the-badge&logo=vercel&logoColor=white)](https://deepnerd.tech)
[![Docs](https://img.shields.io/badge/Docs-deepnerd.tech/docs-0ea5e9?style=for-the-badge&logo=readthedocs&logoColor=white)](https://deepnerd.tech/docs)
[![GitHub Org](https://img.shields.io/badge/GitHub-DeepNerd--AI-181717?style=for-the-badge&logo=github)](https://github.com/DeepNerd-AI)
[![Releases](https://img.shields.io/badge/Releases-Download%20Now-22c55e?style=for-the-badge&logo=github)](https://github.com/DeepNerd-AI/Vault/releases)
[![Discussions](https://img.shields.io/badge/Discussions-Join%20the%20Chat-f59e0b?style=for-the-badge&logo=github)](https://github.com/DeepNerd-AI/Vault/discussions)
[![Issues](https://img.shields.io/badge/Issues-Report%20a%20Bug-ef4444?style=for-the-badge&logo=github)](https://github.com/DeepNerd-AI/Vault/issues)

</div>

---

## 🌐 Language Breakdown

```
Rust            ████████████████████░░  97.9%
Shell           ░░░░░░░░░░░░░░░░░░░░░   0.3%
Python          ░░░░░░░░░░░░░░░░░░░░░   0.3%
Tree-sitter     ░░░░░░░░░░░░░░░░░░░░░   0.4%
Inno Setup      ░░░░░░░░░░░░░░░░░░░░░   0.4%
WGSL            ░░░░░░░░░░░░░░░░░░░░░   0.1%
Other           ░░░░░░░░░░░░░░░░░░░░░   0.6%
```

---

<div align="center">

<img src="https://capsule-render.vercel.app/api?type=waving&color=0:2d0a5e,50:1a0533,100:0d0d0d&height=120&section=footer&animation=twinkling" width="100%" />

**Vault IDE — Built with 🦀 Rust · Powered by GPUI · Made by [DeepNerd-AI](https://github.com/DeepNerd-AI)**

*Part of the [DeepNerd](https://deepnerd.tech) ecosystem — built by [Umang Kumar Singh (UKASITECH)](https://github.com/UKASITECH)*

</div>
