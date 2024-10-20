// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use lib::{on_init_db, on_request_increase_value, AppWindow};
use slint::ComponentHandle;
use tokio::task::spawn_blocking;

#[tokio::main]
async fn main() -> Result<(), slint::PlatformError> {
    spawn_blocking(run_ui).await.unwrap()?;

    Ok(())
}

fn run_ui() -> Result<(), slint::PlatformError> {
    // Initialize UI
    let ui = AppWindow::new()?;

    on_init_db(&ui);
    on_request_increase_value(&ui);

    ui.run()?;

    Ok(())
}
