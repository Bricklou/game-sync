use tauri::{plugin::TauriPlugin, Runtime};
use tauri_plugin_log::{Target, TargetKind};

pub fn register<R: Runtime>() -> TauriPlugin<R> {
    tauri_plugin_log::Builder::default()
        .target(Target::new(TargetKind::Stdout))
        .target(Target::new(TargetKind::Webview))
        .build()
}
