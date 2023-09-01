use tauri::{plugin::TauriPlugin, Runtime};

pub fn register<R: Runtime>() -> TauriPlugin<R> {
    tauri_plugin_store::Builder::default().build()
}
