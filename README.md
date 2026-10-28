# Rust

**English** · [简体中文](https://github.com/Nanobug-Editor/Rust/blob/main/README.zh-CN.md)

Rust recognition and language services, Cargo configurations and the merged debugger. The stable installation ID is `rust`; the first independent release is `0.6.1`.

The current source version is **0.6.2**. Language startup selects the workspace Cargo root or the closest independent plugin project containing the requested document. Unopened independent projects stay outside that server's analysis; opening one creates its own project context with the host SDK available for completion. The readiness deadline remains 120 seconds.

This repository is the authoritative plugin source. The [marketplace registry](https://github.com/Nanobug-Editor/Plugin-Marketplace) reviews submissions; fixed ZIP packages live in this repository's Releases. Publishing a Release does not list it automatically. Every initial submission and version requires a registry PR and human review.

## Build independently

Use Nanobug built from the commit pinned in host-sdk.json, with the required native toolchain and WASM target already available. Set `$pluginHost` to the absolute path of that executable:

```powershell
# The public host entry supplies its SDK and packages the project without a source copy.
& $pluginHost --plugin-package . --output dist
```

For a WASM project, run `& $pluginHost --plugin-cargo Cargo.toml test --lib` as well. Declarative packages need no empty WASM component. Missing tools are reported, never installed automatically. Before publishing, check permissions, platform claims, capability requirements and the SHA-256 of the complete ZIP. Changed bytes require a new version and review.

## Maintenance and licensing

[Original plugin notes](https://github.com/Nanobug-Editor/Rust/blob/v0.6.1/docs/source-readme.md) preserve feature details and prior verification; current source and this README define the independent build. The migration preserves existing IDs, settings and private data. Only the merged Rust package is distributed.

The Nanobug workspace's Cargo MIT declaration is made explicit in LICENSE. Grammar, icons and native dependencies keep their own licenses and notices. Import a generated ZIP through the native manager, or install from the marketplace after review, and approve its actual permissions.
