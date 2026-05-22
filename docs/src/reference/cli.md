---
title: CLI Reference
description: "Reference for Vault's command-line interface (CLI), including opening files and directories, integrating with tools, and controlling Vault from scripts."
---

# CLI Reference

Use Vault's command-line interface (CLI) to open files and directories, integrate with other tools, and control Vault from scripts.

## Installation

**macOS:** Run the `cli: install` command from the command palette ({#kb command_palette::Toggle}) to install the `vault` CLI to `/usr/local/bin/vault`.

**Linux:** The CLI is included with Vault packages. The binary name may vary by distribution (commonly `vault` or `zeditor`).

**Windows:** The CLI is included with Vault. Add Vault's installation directory to your PATH, or use the full path to `vault.exe`.

## Usage

```sh
vault [OPTIONS] [PATHS]...
```

## Opening Files and Directories

Open a file:

```sh
vault myfile.txt
```

Open a directory as a workspace:

```sh
vault ~/projects/myproject
```

Open multiple files or directories:

```sh
vault file1.txt file2.txt ~/projects/myproject
```

Open a file at a specific line and column:

```sh
vault myfile.txt:42        # Open at line 42
vault myfile.txt:42:10     # Open at line 42, column 10
```

## Options

### `-w`, `--wait`

Wait for all opened files to be closed before the CLI exits. When opening a directory, waits until the window is closed.

This is useful for integrating Vault with tools that expect an editor to block until editing is complete (e.g., `git commit`):

```sh
export EDITOR="vault --wait"
git commit  # Opens Vault and waits for you to close the commit message file
```

### `-n`, `--new`

Open paths in a new workspace window, even if the paths are already open in an existing window:

```sh
vault -n ~/projects/myproject
```

### `-a`, `--add`

Add paths to the currently focused workspace instead of opening a new window. When multiple workspace windows are open, files open in the focused window:

```sh
vault -a newfile.txt
```

### `-r`, `--reuse`

Reuse an existing window, replacing its current workspace with the new paths:

```sh
vault -r ~/projects/different-project
```

### `--diff <OLD_PATH> <NEW_PATH>`

Open a diff view comparing two files. Can be specified multiple times:

```sh
vault --diff file1.txt file2.txt
vault --diff old.rs new.rs --diff old2.rs new2.rs
```

### `--foreground`

Run Vault in the foreground, keeping the terminal attached. Useful for debugging:

```sh
vault --foreground
```

### `--user-data-dir <DIR>`

Use a custom directory for all user data (database, extensions, logs) instead of the default location:

```sh
vault --user-data-dir ~/.vault-custom
```

Default locations:

- **macOS:** `~/Library/Application Support/Vault`
- **Linux:** `$XDG_DATA_HOME/vault` (typically `~/.local/share/vault`)
- **Windows:** `%LOCALAPPDATA%\Vault`

### `-v`, `--version`

Print Vault's version and exit:

```sh
vault --version
```

### `--uninstall`

Uninstall Vault and remove all related files (macOS and Linux only):

```sh
vault --uninstall
```

### `--vault <PATH>`

Specify a custom path to the Vault application or binary:

```sh
vault --vault /path/to/Vault.app myfile.txt
```

## Reading from Standard Input

Read content from stdin by passing `-` as the path:

```sh
echo "Hello, World!" | vault -
cat myfile.txt | vault -
ps aux | vault -
```

This creates a temporary file with the stdin content and opens it in Vault.

## URL Handling

The CLI can open `vault://`, `http://`, and `https://` URLs:

```sh
vault vault://settings
vault https://github.com/DeepNerd-AI/vault
```

## Using Vault as Your Default Editor

Set Vault as your default editor for Git and other tools:

```sh
export EDITOR="vault --wait"
export VISUAL="vault --wait"
```

Add these lines to your shell configuration file (e.g., `~/.bashrc`, `~/.zshrc`).

## macOS: Switching Release Channels

On macOS, you can launch a specific release channel by passing the channel name as the first argument:

```sh
vault --stable myfile.txt
vault --preview myfile.txt
vault --nightly myfile.txt
```

## WSL Integration (Windows)

On Windows, the CLI supports opening paths from WSL distributions. This is handled automatically when launching Vault from within WSL.

## Exit Codes

| Code | Meaning                           |
| ---- | --------------------------------- |
| `0`  | Success                           |
| `1`  | Error (details printed to stderr) |

When using `--wait`, the exit code reflects whether the files were saved before closing.
