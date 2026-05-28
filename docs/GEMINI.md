# Gemini CLI Rules

Specific instructions for the **Gemini CLI** agent in the PoshBuddy repository.

## Working Context
- Project follows the Jules Dev Standard.
- Core logic is in Rust (2024 edition).
- Asynchronous runtime: Tokio.
- TUI built with Ratatui and Crossterm.
- E2E testing uses Playwright (Node.js required).

## Workflow
1. Research -> 2. Strategy -> 3. Execution (Plan-Act-Validate).
2. Validate using `cargo test` and `npx playwright test`.
3. Adhere to idiomatic Rust and maintain system safety (backups).

## Constraints
- Never block the UI thread with synchronous I/O.
- Always update `docs/MEMORY.md` after significant architectural changes or milestones.
- Mirror logic between TUI and CLI where applicable (shared services).
- Follow the architectural patterns defined in `docs/wiki/architecture.md`.
- Use the `AppMessage` system for all inter-component communication.