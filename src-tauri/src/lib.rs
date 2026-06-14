mod commands;
mod db;
mod models;

use db::DbState;
use std::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_data_dir = app
                .path()
                .app_data_dir()
                .map_err(|e| e.to_string())?
                .to_string_lossy()
                .to_string();

            std::fs::create_dir_all(&app_data_dir).map_err(|e| e.to_string())?;

            let conn = db::init_db(&app_data_dir)?;
            app.manage(DbState {
                conn: Mutex::new(conn),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::chapter::list_chapters,
            commands::chapter::list_course_outcomes,
            commands::task::list_tasks,
            commands::task::get_task_detail,
            commands::task::get_random_tasks,
            commands::task::list_knowledge_points,
            commands::practice::save_answer,
            commands::practice::assess_requirement,
            commands::practice::assess_task,
            commands::stats::get_chapter_progress,
            commands::stats::get_overall_progress,
            commands::import::import_tasks,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}