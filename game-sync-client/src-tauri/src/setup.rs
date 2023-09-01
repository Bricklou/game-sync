use tauri::{App, Manager};

use crate::{modules::tray, plugins};

pub fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    tray::create_tray(app.handle())?;

    plugins::store::init(&app.handle())?;

    #[cfg(debug_assertions)]
    {
        app.get_window("main").unwrap().open_devtools();
    }

    Ok(())
}
