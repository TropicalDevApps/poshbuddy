# Agent SOP: PoshBuddy Operations

## Operational Mandates
- **Stability:** Rust 2024 edition features should be used judiciously. Maintain compatibility with stable toolchains.
- **Safety:** Any change that modifies `$PROFILE` or theme JSONs must be preceded by a call to the `backup` service.
- **Async Hygiene:** Do not block the Ratatui event loop. Always use `tokio::spawn` for I/O or heavy computations and communicate via `mpsc` channels.

## Core Workflows
1. **Modifying the TUI:**
   - Update the state model in `src/app/models.rs`.
   - Implement rendering logic in `src/ui.rs`.
   - Wire event handling in `src/app/handlers.rs`.
2. **Adding a CLI Command:**
   - Define the command/args in `src/cli.rs` using Clap.
   - Implement the business logic in `src/app/services.rs`.
   - Add integration tests using the Rust test suite.
3. **Updating Theme Logic:**
   - Modify the `ThemeAsset` engine or segment toggling logic in `src/app/services.rs`.
   - Verify changes against real Oh My Posh schemas.

## Documentation SOP
- Update `CHANGELOG.md` for every release or significant feature.
- Maintain the wiki in `docs/wiki/`.
- Keep `docs/MEMORY.md` updated with the latest project status.

## Related Docs
- [Project Identity](./IDENTITY.md)
- [Project Soul](./SOUL.md)
- [Wiki Index](./wiki/index.md)