use tauri::{plugin::TauriPlugin, AppHandle, Manager, Runtime};
use tauri_plugin_store::StoreBuilder;

pub fn register<R: Runtime>() -> TauriPlugin<R> {
    tauri_plugin_store::Builder::default().build()
}

pub fn init<R: Runtime>(app_handle: &AppHandle<R>) -> Result<(), Box<dyn std::error::Error>> {
    let store_path = app_handle.path().config_dir()?.join("store.bin");
    let store = StoreBuilder::new(store_path).build(app_handle.clone());

    app_handle.manage(store);

    Ok(())
}
