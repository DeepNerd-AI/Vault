# Contributing to Vault

Thank you for helping us make Vault better!

All activity in Vault forums is subject to our [Code of Conduct](https://deepnerd.tech/code-of-conduct). Additionally, contributors must sign our Contributor License Agreement before their contributions can be merged.

## Contribution ideas

Vault is a large project with a number of priorities. We spend most of our time working on what we believe the product needs, but we also love working with the community to improve the product in ways we haven't thought of (or had time to get to yet!)

In particular we love PRs that are:

- Fixing or extending the docs.
- Fixing bugs.
- Small enhancements to existing features to make them work for more people (making things work on more platforms/modes/whatever).
- Small extra features, like keybindings or actions you miss from other editors or extensions.

If you're looking for concrete ideas, check out our GitHub Issues where we track bugs, enhancement requests, and planned features.

## Sending changes

The Vault culture values working code and synchronous conversations over long discussion threads.

The best way to get us to take a look at a proposed change is to send a pull request. We will get back to you as soon as possible.

Although we will take a look, we tend to only merge PRs that align with our roadmap and code standards. If you'd like your PR to have the best chance of being merged:

- Make sure the change is **desired**: we're always happy to accept bugfixes, but features should be confirmed with us first if you aim to avoid wasted effort.
- Include a clear description of **what you're solving**, and why it's important.
- Include **tests**. For UI changes, consider updating visual tests.
- If it changes the UI, attach **screenshots** or screen recordings.
- Make the PR about **one thing only**, e.g. if it's a bugfix, don't add two features and a refactoring on top of that.
- Keep AI assistance under your judgement and responsibility: it's unlikely we'll merge a vibe-coded PR that the author doesn't fully understand.

### UI/UX checklist

When your changes affect UI, consult this checklist:

**Accessibility / Ergonomics**
- Do all keyboard shortcuts work as intended?
- Are shortcuts discoverable (tooltips, menus, docs)?
- Do all mouse actions work (drag, context menus, resizing, scrolling)?
- Does the feature look great in light mode and dark mode?
- Are hover states, focus rings, and active states clear and consistent?
- Is it usable without a mouse (keyboard-only navigation)?

**Responsiveness**
- Does the UI scale gracefully on:
    - Narrow panes (e.g., side-by-side split views)?
    - Short panes (e.g., laptops with 13" displays)?
    - High-DPI / Retina displays?
- Does resizing panes or windows keep the UI usable and attractive?
- Do dialogs or modals stay centered and within viewport bounds?

**Platform Consistency**
- Is the feature fully usable on Linux? (And compiles cleanly on Windows/macOS if building from source?)
- Does it respect system-level settings (fonts, scaling, input methods)?

**Performance**
- All user interactions must have instant feedback.
    - If the user requests something slow (e.g. an LLM generation) there should be some indication of the work in progress.
- Does it handle large files, big projects, or heavy workloads without degrading?
- Frames must take no more than 8ms (120fps)

**Consistency**
- Does it match Vault’s design language (spacing, typography, icons)?
- Are terminology, labels, and tone consistent with the rest of Vault?
- Are interactions consistent (e.g., how tabs close, how modals dismiss, how errors show)?

**Internationalization & Text**
- Are strings concise, clear, and unambiguous?
- Do we avoid internal jargon that only insiders would know?

**User Paths & Edge Cases**
- What does the happy path look like?
- What does the unhappy path look like? (errors, rejections, invalid states)
- How does it work in offline vs. online states?
- How does it behave if data is missing, corrupted, or delayed?
- Are error messages actionable and consistent with Vault’s voice?

**Discoverability & Learning**
- Can a first-time user figure it out without docs?
- Is there an intuitive way to undo/redo actions?
- Are power features discoverable but not intrusive?
- Is there a path from beginner → expert usage (progressive disclosure)?

## Things we will (probably) not merge

Although there are few hard and fast rules, typically we don't merge:

- Anything that can be provided by an extension. For example a new language, or theme.
- Features where (in our subjective opinion) the extra complexity isn't worth it for the number of people who will benefit.
- Giant refactorings.
- Non-trivial changes with no tests.
- Stylistic code changes that do not alter any app logic. Reducing allocations, removing `.unwrap()`s, fixing typos is great; making code "more readable" — maybe not so much.
- Anything that seems AI-generated without understanding the output.

## Bird's-eye view of Vault

We suggest you keep the glossary and docs at hand when starting out. Vault is made up of several smaller crates - let's go over those you're most likely to interact with:

- [`gpui`](/crates/gpui) is a GPU-accelerated UI framework which provides all of the building blocks for Vault.
- [`editor`](/crates/editor) contains the core `Editor` type that drives both the code editor and all various input fields within Vault.
- [`project`](/crates/project) manages files and navigation within the filetree.
- [`workspace`](/crates/workspace) handles local state serialization and groups projects together.
- [`lsp`](/crates/lsp) handles communication with external LSP servers.
- [`language`](/crates/language) drives `editor`'s understanding of languages.
- [`theme`](/crates/theme) defines the theme system and provides default themes.
- [`ui`](/crates/ui) is a collection of UI components and common patterns used throughout Vault.
- [`cli`](/crates/cli) is the CLI crate which invokes the Vault binary.
- [`vault`](/crates/vault) is where all things come together, and the main entry point for the editor.

## Packaging Vault

Check our [notes for packaging Vault](https://deepnerd.tech/docs/development/linux).
