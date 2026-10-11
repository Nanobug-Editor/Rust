//! One Rust package routes language, Cargo and debugger policy over the public host SDK.
mod cargo;
mod debugger;
mod language;
use plugin_protocol::{
    api::{self, Failure},
    bindings::{Guest, export},
    service,
};

struct RustPlugin;
impl Guest for RustPlugin {
    /// Service contracts select policy; asynchronous notifications retain their resource ownership.
    fn dispatch(payload: String) -> Result<String, String> {
        api::guest::dispatch(&payload, dispatch)
    }
}
export!(RustPlugin);

/// Shared notifications reach both state owners; each consumes only its own exact handles.
fn dispatch(input: api::Input) -> Result<api::Output, Failure> {
    match input {
        api::Input::Event {
            event: api::Notification::Service(service::Notification::Invoke(call)),
            panel,
        } => match call.contract.as_str() {
            plugin_protocol::configurations::CONTRACT => cargo::configurations::invoke(call),
            plugin_protocol::targets::CONTRACT => cargo::targets::dispatch(api::Input::Event {
                event: api::Notification::Service(service::Notification::Invoke(call)),
                panel,
            }),
            "debug.session" => debugger::dispatch(api::Input::Event {
                event: api::Notification::Service(service::Notification::Invoke(call)),
                panel,
            }),
            _ => Err(Failure::new(
                api::ErrorCode::UnsupportedOperation,
                "Unknown Rust service contract",
            )),
        },
        api::Input::Event {
            event: api::Notification::LanguageService(context),
            ..
        } => Ok(api::Output {
            language_service: Some(language::prepare_language(context)?),
            ..Default::default()
        }),
        input @ api::Input::Event {
            event: api::Notification::Process { .. } | api::Notification::Service(_),
            ..
        } => {
            cargo::targets::dispatch(input.clone())?;
            debugger::dispatch(input)
        }
        // Logical snapshots never replay Cargo builds, adapters or target processes.
        api::Input::Snapshot => Ok(api::Output {
            snapshot: Some(Default::default()),
            ..Default::default()
        }),
        _ => Ok(Default::default()),
    }
}
