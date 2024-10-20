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
            // TODO: fix empty content bug
            let db: JsonDB<Todo> = JsonDB::new(&db_name).await.unwrap();
            let db_path = db.get_db_path().to_string();
            // db.add_table("users").await.unwrap();
            // db.add_table("products").await.unwrap();
            let tables = db.get_db_tables().await;

            ui_handle.upgrade_in_event_loop(move |ui| {
                let tables_model = Rc::new(slint::VecModel::from(
                    tables
                        .into_iter()
                        .map(|s| s.into())
                        .collect::<Vec<slint::SharedString>>(),
                ));

                let cfg = ui.global::<AppConfig>();
                cfg.set_db_path(db_path.into());
                cfg.set_tables(tables_model.into());
                cfg.set_is_rocket_launched(!cfg.get_is_rocket_launched());
            })
        });
    });
}
