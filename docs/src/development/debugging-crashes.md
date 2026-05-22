---
title: Debugging Crashes
description: "Guide to debugging crashes for Vault development."
---

# Debugging Crashes

When Vault panics or crashes, it sends a message to a sidecar process that inspects the editor's memory and creates a [minidump](https://chromium.googlesource.com/breakpad/breakpad/+/master/docs/getting_started_with_breakpad.md#the-minidump-file-format) in `~/Library/Logs/Vault` or `$XDG_DATA_HOME/vault/logs`. You can use this minidump to generate backtraces for all thread stacks.

If telemetry is enabled, Vault uploads these reports when you restart the app. Reports are sent to a [Slack channel](https://DeepNerd-AI.slack.com/archives/C0977J9MA1Y) and to [Sentry](https://vault-dev.sentry.io/issues) (both are Vault-staff-only).

These crash reports include useful data, but they are hard to read without spans or symbol information. You can still analyze them locally by downloading source and an unstripped binary (or separate symbols file) for your Vault release, then running:

```sh
zstd -d ~/.local/share/vault/<uuid>.dmp -o minidump.dmp
minidump-stackwalk minidump.dmp
```

Alongside the minidump in your logs directory, you should also see a `<uuid>.json` file with metadata such as the panic message, span, and system specs.

## Using a Debugger

If you can reproduce the crash consistently, use a debugger to inspect program state at the crash point.

For setup details, see [Using a debugger](./debuggers.md#debugging-panics-and-crashes).
