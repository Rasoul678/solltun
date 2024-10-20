use std::rc::Rc;

use crate::{AppConfig, AppLogic, AppWindow, Todo};
use ohmydb::JsonDB;
use slint::{ComponentHandle, Timer, TimerMode};

pub fn on_init_db(ui: &AppWindow) {
    let logic = ui.global::<AppLogic>();
    let ui_handle = ui.as_weak();

    logic.on_init_db(move || {
        let ui = ui_handle.upgrade().unwrap();
        let ui_handle = ui.as_weak();

        let timer = Timer::default();
        timer.start(
            TimerMode::SingleShot,
            std::time::Duration::from_millis(1000),
            move || {
                println!("Rocket will be launched after 1000ms.");
            },
        );

        let cfg = ui.global::<AppConfig>();
        let db_name = cfg.get_db_name();

        tokio::runtime::Handle::current().spawn(async move {
            let mut db: JsonDB<Todo> = JsonDB::new(&db_name).await.unwrap();
            let db_path = db.get_db_path().to_string();
            db.add_table("users").await.unwrap();
            db.add_table("products").await.unwrap();
            let tables = db.get_db_tables().await;

            ui_handle.upgrade_in_event_loop(move |ui| {
                let tables_model = Rc::new(slint::VecModel::from(
                    tables
                        .into_iter()
                        .map(|s| s.into())
                        .collect::<Vec<slint::SharedString>>(),
                ));

                ui.set_db_name(db_name.into());
                ui.set_db_path(db_path.into());
                ui.set_tables(tables_model.into());
                ui.set_is_rocket_launched(!ui.get_is_rocket_launched());
            })
        });
    });
}
pub fn on_request_increase_value(ui: &AppWindow) {
    let ui_handle = ui.as_weak();

    ui.on_request_increase_value({
        let ui = ui_handle.unwrap();

        move || {
            ui.set_counter(ui.get_counter() + 1);
        }
    });
}
