
# Vault

[![Vault](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/DeepNerd-AI/Vault/main/assets/badge/v0.json)](https://deepnerd.tech)
[![CI](https://github.com/DeepNerd-AI/Vault/actions/workflows/run_tests.yml/badge.svg)](https://github.com/DeepNerd-AI/Vault/actions/workflows/run_tests.yml)

Welcome to Vault, a high-performance, multiplayer code editor from the creators of [CodePlanet](https://github.com/codeplanetlabs) and [Zed](https://github.com/zed-industries). It is a fork of the Zed Code Editor, But Better.

---


### Installation

On macOS, Linux, and Windows you can [download Vault directly](https://deepnerd.tech/download) or install Vault via your local package manager ([macOS](https://deepnerd.tech/docs/installation#macos)/[Linux](https://deepnerd.tech/docs/linux#installing-via-a-package-manager)/[Windows](https://deepnerd.tech/docs/windows#package-managers)).

Other platforms are not yet available:

- Web ([tracking issue](https://github.com/zed-industries/zed/issues/5396))

### Developing Vault

- [Building Vault for macOS](./docs/src/development/macos.md)
- [Building Vault for Linux](./docs/src/development/linux.md)
- [Building Vault for Windows](./docs/src/development/windows.md)

### Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for ways you can contribute to Vault.

Also... we're hiring! Check out our [jobs](https://deepnerd.tech/jobs) page for open roles.

### Licensing

License information for third party dependencies must be correctly provided for CI to pass.

We use [`cargo-about`](https://github.com/EmbarkStudios/cargo-about) to automatically comply with open source licenses. If CI is failing, check the following:

- Is it showing a `no license specified` error for a crate you've created? If so, add `publish = false` under `[package]` in your crate's Cargo.toml.
- Is the error `failed to satisfy license requirements` for a dependency? If so, first determine what license the project has and whether this system is sufficient to comply with this license's requirements. If you're unsure, ask a lawyer. Once you've verified that this system is acceptable add the license's SPDX identifier to the `accepted` array in `script/licenses/zed-licenses.toml`.
- Is `cargo-about` unable to find the license for a dependency? If so, add a clarification field at the end of `script/licenses/zed-licenses.toml`, as specified in the [cargo-about book](https://embarkstudios.github.io/cargo-about/cli/generate/config.html#crate-configuration).

## Sponsorship

Vault is developed by **DeepNerd-AI**, a for-profit company.

If you’d like to financially support the project, you can do so via GitHub Sponsors.
Sponsorships go directly to DeepNerd-AI and are used as general company revenue.
There are no perks or entitlements associated with sponsorship.
