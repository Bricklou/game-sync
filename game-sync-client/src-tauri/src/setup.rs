use tauri::App;

use crate::modules::tray;

pub fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    tray::create_tray(app.handle())?;

    Ok(())
}
