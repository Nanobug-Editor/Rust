# Rust 0.6.3

- Preserve comment colors for `///` and `//!` markers in both themes.
- Cover qualified/generic calls, macros, lifetimes, declared constants, path keywords and compound operators without changing the packaged grammar.
- Keep installation identity, settings, permissions and language startup behavior unchanged. See [verification](docs/verification/highlighting-2026-10-30.md).

## Rust 0.6.2

- Select the Cargo project belonging to the requested document instead of loading every independent plugin under a workspace at startup.
- Keep excluded nested plugin sources in their own language context with the public host SDK available; preserve the 120-second readiness deadline.
- Require `language.lsp >=1.5,<2` for document-scoped discovery. Installation ID, settings and permissions remain unchanged.
