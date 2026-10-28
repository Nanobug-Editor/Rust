# Rust 0.6.2

- Select the Cargo project belonging to the requested document instead of loading every independent plugin under a workspace at startup.
- Keep excluded nested plugin sources in their own language context with the public host SDK available; preserve the 120-second readiness deadline.
- Require `language.lsp >=1.5,<2` for document-scoped discovery. Installation ID, settings and permissions remain unchanged.
