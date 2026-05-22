---
title: Building Vault for FreeBSD
description: "Guide to building vault for freebsd for Vault development."
---

# Building Vault for FreeBSD

FreeBSD is not currently a supported platform, so this guide is a work in progress.

## Repository

Clone the [Vault repository](https://github.com/DeepNerd-AI/vault).

## Dependencies

- Install the necessary system packages and rustup:

  ```sh
  script/freebsd
  ```

  If preferred, you can inspect [`script/freebsd`](https://github.com/DeepNerd-AI/vault/blob/main/script/freebsd) and perform the steps manually.

## Building from source

Once the dependencies are installed, you can build Vault using [Cargo](https://doc.rust-lang.org/cargo/).

For a debug build of the editor:

```sh
cargo run
```

And to run the tests:

```sh
cargo test --workspace
```

In release mode, the primary user interface is the `cli` crate. You can run it in development with:

```sh
cargo run -p cli
```

### WebRTC Notice

Building `webrtc-sys` on FreeBSD currently fails due to missing upstream support and unavailable prebuilt binaries. As a result, collaboration features that depend on WebRTC (audio calls and screen sharing) are temporarily disabled.

See [Issue #15309: FreeBSD Support] and [Discussion #29550: Unofficial FreeBSD port for Vault] for more.

## Troubleshooting

### Cargo errors claiming that a dependency is using unstable features

Try `cargo clean` and `cargo build`.
