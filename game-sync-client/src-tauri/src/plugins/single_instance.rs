use tauri::{plugin::TauriPlugin, Manager, Runtime};

#[derive(Clone, serde::Serialize)]
pub struct Payload {
    args: Vec<String>,
    cwd: String,
}

pub fn register<R: Runtime>() -> TauriPlugin<R> {
    tauri_plugin_single_instance::init(|app, argv, cwd| {
        println!("{}, {argv:?}, {cwd:?}", app.package_info().name);

        app.emit_all("single-instance", Payload { args: argv, cwd })
            .unwrap();
    })
}
