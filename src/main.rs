// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use slint::ComponentHandle;
use solltun::{on_init_db, on_request_increase_value, AppWindow};
use tokio::task::spawn_blocking;

#[tokio::main]
async fn main() -> Result<(), slint::PlatformError> {
    spawn_blocking(run_ui).await.unwrap()?;

    Ok(())
}

fn run_ui() -> Result<(), slint::PlatformError> {
    // Initialize UI
    let ui = AppWindow::new()?;
    // Get a UI weakref for use in the callback
    let ui_handle = ui.as_weak();

    on_init_db(ui_handle.unwrap()).unwrap();
    on_request_increase_value(ui_handle.unwrap()).unwrap();

    ui.run()?;

    Ok(())
}
