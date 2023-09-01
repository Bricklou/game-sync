use tauri::{Runtime, State};
use tauri_plugin_store::Store;

#[derive(serde::Serialize)]
pub struct ConfiguredServer {
    pub url: Option<String>,
}

#[tauri::command]
pub fn get_configured_server<R: Runtime>(
    _: tauri::AppHandle<R>,
    store: State<'_, Store<R>>,
) -> Result<ConfiguredServer, String> {
    let store = store.inner();

    if let Some(server_url) = store.get("server_url") {
        return Ok(ConfiguredServer {
            url: Some(server_url.to_string()),
        });
    }

    Ok(ConfiguredServer { url: None })
}
