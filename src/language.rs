//! Rust Analyzer discovery and configuration remain language-owned plugin policy.
use plugin_protocol::{
    api::{self, ErrorCode, Failure},
    language,
};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};

/// Discover project manifests for the requested document, preserving ignored-tree exclusions.
pub(super) fn prepare_language(context: language::Context) -> Result<language::Proposal, Failure> {
    // Installation asks only for dependency plans. Rust Analyzer uses local tools and has no
    // download plan, so SDK/project discovery stays on the actual language-start path.
    if context.preparing_dependencies {
        return Ok(Default::default());
    }
    let sdk = match api::guest::request(api::Operation::DescribeSdk)? {
        api::Value::Sdk(sdk) => sdk,
        _ => {
            return Err(Failure::new(
                ErrorCode::InvalidRequest,
                "Expected host SDK descriptor",
            ));
        }
    };
    let workspace = api::guest::open_workspace()?;
    let result = api::guest::request(api::Operation::FindFiles {
        handle: workspace.clone(),
        query: api::FileQuery {
            include: [
                "Cargo.toml",
                "manifest.json",
                "**/Cargo.toml",
                "**/manifest.json",
            ]
            .map(str::to_owned)
            .to_vec(),
            exclude: [
                "**/target/**",
                "**/vendor/**",
                "**/node_modules/**",
                "**/.git/**",
            ]
            .map(str::to_owned)
            .to_vec(),
            max_results: 4096,
        },
    });
    // Discovery handles never survive the pure preparation callback, including host request failures.
    let _ = api::guest::close_resource(workspace);
    let matches = match result? {
        api::Value::Files(files) => files,
        _ => {
            return Err(Failure::new(
                ErrorCode::InvalidRequest,
                "Expected workspace file matches",
            ));
        }
    };
    // Unreadable unrelated directories are reported by the host but do not disable all language analysis.
    Ok(language_proposal(
        &context.workspace,
        &sdk.cargo_config,
        &matches.paths,
        context.document.as_ref(),
    ))
}

/// Match Rust Analyzer's URI/VFS spelling even though the guest itself runs under WASI rather than Windows.
fn analysis_path(path: &str) -> String {
    let mut path = path.replace('\\', "/");
    if let Some(unc) = path.strip_prefix("//?/UNC/") {
        path = format!("//{unc}");
    } else if let Some(ordinary) = path.strip_prefix("//?/") {
        path = ordinary.to_owned();
    }
    if path.as_bytes().get(1) == Some(&b':') && path.as_bytes()[0].is_ascii_alphabetic() {
        path[..1].make_ascii_lowercase();
    }
    path
}

/// Select the closest paired plugin containing `document`, falling back to the workspace Cargo root.
/// Without a document, `matches` may select only the root; unrelated plugins never enter startup.
fn project_manifest<'a>(matches: &'a [String], document: Option<&str>) -> Option<&'a str> {
    let paths: BTreeSet<_> = matches.iter().map(String::as_str).collect();
    paths
        .iter()
        .copied()
        .filter(|path| {
            path.strip_suffix("Cargo.toml").is_some_and(|directory| {
                directory.is_empty()
                    || document.is_some_and(|document| {
                        document.starts_with(directory)
                            && paths.contains(format!("{directory}manifest.json").as_str())
                    })
            })
        })
        .max_by_key(|path| path.len())
}

/// Build SDK-aware options for one selected manifest in `workspace`, leaving project files untouched.
/// `cargo_config` is the host-owned SDK override; absence of a manifest preserves non-Cargo folders.
fn initialization_options(workspace: &str, cargo_config: &str, manifest: Option<&str>) -> Value {
    let mut options = json!({"cargo":{"configPath":analysis_path(cargo_config)},
        "diagnostics":{"enable":true,"experimental":{"enable":true}}});
    if let Some(manifest) = manifest {
        options["linkedProjects"] = json!([analysis_path(&format!(
            "{}/{}",
            workspace.trim_end_matches(['/', '\\']),
            manifest
        ))]);
    }
    options
}

/// A document shares a service only with its selected Cargo root, retaining isolated SDK analysis.
/// `matches` are ignore-aware workspace paths; the native host validates the proposed relative root.
fn language_proposal(
    workspace: &str,
    cargo_config: &str,
    matches: &[String],
    document: Option<&language::DocumentContext>,
) -> language::Proposal {
    let manifest = project_manifest(matches, document.map(|document| document.path.as_str()));
    let options = initialization_options(workspace, cargo_config, manifest);
    language::Proposal {
        // The host keys transports by provider, canonical root and this bounded project identity.
        // Root files reuse one service while excluded plugins keep their own native analysis context.
        context: document.map(|_| language::ProjectContext {
            key: "cargo".into(),
            generation: 0,
        }),
        project_root: manifest.and_then(|path| path.rsplit_once('/').map(|(root, _)| root.into())),
        configuration: Some(configuration_sections(&options)),
        initialization_options: Some(options),
        ..Default::default()
    }
}

/// Publish complete and dotted Rust Analyzer sections as opaque generic configuration keys.
fn configuration_sections(options: &Value) -> BTreeMap<String, Value> {
    fn visit(prefix: &str, value: &Value, sections: &mut BTreeMap<String, Value>) {
        sections.insert(prefix.to_owned(), value.clone());
        if let Some(fields) = value.as_object() {
            for (key, value) in fields {
                visit(&format!("{prefix}.{key}"), value, sections);
            }
        }
    }
    let mut sections = BTreeMap::from([(String::new(), options.clone())]);
    visit("rust-analyzer", options, &mut sections);
    sections
}

#[cfg(test)]
mod tests {
    use super::*;
    /// Cold workspace discovery must not start Cargo analysis for unrelated independent plugins.
    #[test]
    fn workspace_startup_does_not_link_unopened_plugin_projects() {
        let files = [
            "Cargo.toml",
            "plugins/unused/Cargo.toml",
            "plugins/unused/manifest.json",
        ]
        .map(str::to_owned);
        let options = language_proposal("/workspace", "/sdk/config.toml", &files, None)
            .initialization_options
            .unwrap();
        assert_eq!(options["linkedProjects"], json!(["/workspace/Cargo.toml"]));
    }

    /// Opening a plugin source selects only that plugin and preserves complete SDK configuration.
    #[test]
    fn paired_projects_and_configuration_remain_guest_policy() {
        let files = [
            "Cargo.toml",
            "plugins/demo/Cargo.toml",
            "plugins/demo/manifest.json",
            "plugins/unused/Cargo.toml",
            "plugins/unused/manifest.json",
            "unrelated/Cargo.toml",
        ]
        .map(str::to_owned);
        let document = language::DocumentContext {
            path: "plugins/demo/src/lib.rs".into(),
            generation: 1,
        };
        let proposal = language_proposal(
            r"\\?\C:\Projects\demo",
            r"\\?\C:\Cache\sdk.toml",
            &files,
            Some(&document),
        );
        assert_eq!(proposal.project_root.as_deref(), Some("plugins/demo"));
        assert_eq!(proposal.context.unwrap().key, "cargo");
        let options = proposal.initialization_options.unwrap();
        assert_eq!(
            options["linkedProjects"],
            json!(["c:/Projects/demo/plugins/demo/Cargo.toml"])
        );
        let sections = configuration_sections(&options);
        assert_eq!(
            sections["rust-analyzer.cargo.configPath"],
            "c:/Cache/sdk.toml"
        );
        assert_eq!(
            sections["rust-analyzer.diagnostics.experimental.enable"],
            true
        );
        assert_eq!(sections[""], options);
        assert_eq!(sections["rust-analyzer"], options);
    }
    /// A UNC root stays absolute and a non-Cargo folder never gets a synthetic linked project.
    #[test]
    fn unc_and_empty_discovery_are_preserved() {
        assert_eq!(
            analysis_path(r"\\?\UNC\server\share\Cargo.toml"),
            "//server/share/Cargo.toml"
        );
        assert!(
            initialization_options("/workspace", "/sdk/config.toml", None)
                .get("linkedProjects")
                .is_none()
        );
    }

    /// Native workspace documents and unopened folders exclude every unrelated paired project.
    #[test]
    fn document_selection_preserves_root_boundaries_and_nearest_plugin() {
        let files = [
            "Cargo.toml",
            "plugins/demo/Cargo.toml",
            "plugins/demo/manifest.json",
            "plugins/demo/nested/Cargo.toml",
            "plugins/demo/nested/manifest.json",
            "unpaired/Cargo.toml",
        ]
        .map(str::to_owned);
        for (document, expected) in [
            (None, "Cargo.toml"),
            (Some("src/lib.rs"), "Cargo.toml"),
            (Some("plugins/demo-extra/src/lib.rs"), "Cargo.toml"),
            (
                Some("plugins/demo/src/unsaved.rs"),
                "plugins/demo/Cargo.toml",
            ),
            (
                Some("plugins/demo/nested/src/lib.rs"),
                "plugins/demo/nested/Cargo.toml",
            ),
            (Some("unpaired/src/lib.rs"), "Cargo.toml"),
        ] {
            assert_eq!(project_manifest(&files, document), Some(expected));
        }
    }

    /// A folder containing independent plugins acquires only the project of its opened source.
    #[test]
    fn non_cargo_workspace_keeps_unopened_plugins_cold() {
        let files = ["plugins/demo/Cargo.toml", "plugins/demo/manifest.json"].map(str::to_owned);
        assert_eq!(project_manifest(&files, None), None);
        assert_eq!(project_manifest(&files, Some("loose.rs")), None);
        assert_eq!(
            project_manifest(&files, Some("plugins/demo/src/lib.rs")),
            Some("plugins/demo/Cargo.toml")
        );
        assert!(
            language_proposal("/workspace", "/sdk/config.toml", &files, None)
                .context
                .is_none()
        );
    }

    /// Workspace tabs share one stable context while excluded plugin tabs receive another root.
    #[test]
    fn document_generations_do_not_split_a_shared_project_context() {
        let files = [
            "Cargo.toml",
            "plugins/demo/Cargo.toml",
            "plugins/demo/manifest.json",
        ]
        .map(str::to_owned);
        let proposal = |path: &str, generation| {
            language_proposal(
                "/workspace",
                "/sdk/config.toml",
                &files,
                Some(&language::DocumentContext {
                    path: path.into(),
                    generation,
                }),
            )
        };
        let first = proposal("src/lib.rs", 1);
        let second = proposal("src/other.rs", 9);
        assert_eq!(first.context, second.context);
        assert_eq!(first.project_root, second.project_root);
        assert_eq!(first.initialization_options, second.initialization_options);
        assert!(first.project_root.is_none());
        let nested = proposal("plugins/demo/src/lib.rs", 1);
        assert_eq!(nested.project_root.as_deref(), Some("plugins/demo"));
        assert_ne!(first.project_root, nested.project_root);
    }
}
