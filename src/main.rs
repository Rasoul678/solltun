// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use ohmydb::JsonDB;
use tokio::task::spawn_blocking;

slint::include_modules!();

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

    ui.set_app_name("Solltun".into());

    on_request_increase_value(ui_handle.unwrap()).unwrap();

    on_add_db(ui_handle.unwrap()).unwrap();

    ui.run()?;

    Ok(())
}

fn on_add_db(ui: AppWindow) -> Result<(), slint::PlatformError> {
    let ui_handle = ui.as_weak();

    ui.on_add_db(move || {
        let ui = ui_handle.unwrap();
        let ui_handle = ui.as_weak();

        tokio::runtime::Handle::current().spawn(async move {
            let mut db = JsonDB::new().await.unwrap();
            db.add_table("todos".into()).await.unwrap();

            ui_handle.upgrade_in_event_loop(|ui| {
                ui.set_db_name("db".into());
            })
        });
    });

    Ok(())
}

fn on_request_increase_value(ui: AppWindow) -> Result<(), slint::PlatformError> {
    let ui_handle = ui.as_weak();

    ui.on_request_increase_value({
        let ui = ui_handle.unwrap();

        move || {
            ui.set_counter(ui.get_counter() + 1);
        }
    });

    Ok(())
}
