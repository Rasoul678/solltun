// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use ohmydb::JsonDB;
use slint::{Timer, TimerMode};
use solltun::Todo;
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

    on_init_db(ui_handle.unwrap()).unwrap();
    on_request_increase_value(ui_handle.unwrap()).unwrap();

    ui.run()?;

    Ok(())
}

fn on_init_db(ui: AppWindow) -> Result<(), slint::PlatformError> {
    let ui_handle = ui.as_weak();

    ui.on_init_db(move || {
        let ui = ui_handle.unwrap();
        let ui_handle = ui.as_weak();

        let timer = Timer::default();
        timer.start(
            TimerMode::SingleShot,
            std::time::Duration::from_millis(1000),
            move || {
                println!("Rocket will be launched after 1000ms.");
            },
        );

        tokio::runtime::Handle::current().spawn(async move {
            let db_name = "ohmytodos";
            let db: JsonDB<Todo> = JsonDB::new(db_name).await.unwrap();
            let db_path = db.get_db_path().to_string();

            ui_handle.upgrade_in_event_loop(move |ui| {
                ui.set_app_name("SOLLTUN".into());
                ui.set_db_name(db_name.into());
                ui.set_db_path(db_path.into());
                ui.set_is_rocket_launched(!ui.get_is_rocket_launched());
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
