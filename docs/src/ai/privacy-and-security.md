---
title: AI Privacy and Security - Vault
description: "Vault's approach to AI privacy: opt-in data sharing by default, zero-data retention with providers, and full open-source transparency."
---

# Privacy and Security

## Philosophy

Vault collects minimal data necessary to serve and improve our product. Features that could share data, like AI and telemetry, are either opt-in or can be disabled.

- **Telemetry**: Vault collects only the data necessary to understand usage and fix issues. Client-side telemetry can be disabled in settings.

- **AI**: Data sharing for AI improvement is opt-in, and each share is a one-time action; it does not grant permission for future data collection. You can use Vault's AI features without sharing any data with Vault and without authenticating.

- **Open-Source**: Vault's codebase is public. You can inspect exactly what data is collected and how it's handled. If you find issues, we encourage you to report them.

- **Secure-by-default**: Designing Vault and our Service with "secure-by-default" as an objective is of utmost importance to us. We take your security and ours very seriously and strive to follow industry best-practice in order to uphold that principle.

## Related Documentation

- [Tool Permissions](./tool-permissions.md): Configure granular rules to control which agent actions are auto-approved, blocked, or require confirmation.

- [Worktree trust](../worktree-trust.md): How Vault opens files and directories in restricted mode.

- [Telemetry](../telemetry.md): How Vault collects general telemetry data.

- [Vault AI Features and Privacy](./ai-improvement.md): An overview of Vault's AI features, your data when using AI in Vault, and how to opt-in and help Vault improve these features.

- [Accounts](../authentication.md): When and why you'd need to authenticate into Vault, how to do so, and what scope we need from you.

- [Collab](https://deepnerd.tech/faq#data-and-privacy): How Vault's live collaboration works and how data flows. Vault does not store your code.

## Legal Links

- [Terms of Service](https://deepnerd.tech/terms)
- [Privacy Policy](https://deepnerd.tech/privacy-policy)
- [Vault's Contributor License and Feedback Agreement](https://deepnerd.tech/cla)
- [Subprocessors](https://deepnerd.tech/subprocessors)
