mod commands;
mod db;
mod errors;
mod models;

use db::DbState;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let study_data_dir = app
                .path()
                .app_local_data_dir()
                .map_err(|e| e.to_string())?
                .join("study-data");

ensure_not_on_desktop(app, &study_data_dir)?;
    std::fs::create_dir_all(&study_data_dir).map_err(|e| e.to_string())?;
    let migrated = migrate_legacy_desktop_db(app, &study_data_dir)?;
    if migrated {
        eprintln!("[迁移] 已将桌面旧版数据复制到新目录，可手动删除桌面上的「计组备考助手」文件夹。");
    }
    #[cfg(debug_assertions)]
    write_path_diagnostics(app, &study_data_dir);

            let conn = db::init_db(&study_data_dir)?;
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
            commands::reset::reset_progress,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn ensure_not_on_desktop(app: &tauri::App, path: &Path) -> Result<(), String> {
    if let Ok(desktop_dir) = app.path().desktop_dir() {
        if path.starts_with(&desktop_dir) {
            eprintln!(
                "[严重] 数据目录 {} 位于桌面 {} 下，已拒绝启动",
                path.display(),
                desktop_dir.display()
            );
            return Err(format!(
                "应用数据目录异常：{} 位于桌面目录 {} 下，已拒绝启动以避免污染桌面。",
                path.display(),
                desktop_dir.display()
            ));
        }
    }
    Ok(())
}

fn migrate_legacy_desktop_db(app: &tauri::App, study_data_dir: &Path) -> Result<bool, String> {
    let new_db = study_data_dir.join("study.db");
    if new_db.exists() {
        return Ok(false);
    }

    let desktop_dir = match app.path().desktop_dir() {
        Ok(path) => path,
        Err(_) => return Ok(false),
    };
    let legacy_dir = desktop_dir.join("计组备考助手");
    let legacy_db = legacy_dir.join("study.db");
    if !legacy_db.exists() {
        return Ok(false);
    }

    std::fs::create_dir_all(study_data_dir).map_err(|e| e.to_string())?;
    std::fs::copy(&legacy_db, &new_db).map_err(|e| e.to_string())?;

    for suffix in ["study.db-wal", "study.db-shm"] {
        let old_file = legacy_dir.join(suffix);
        if old_file.exists() {
            let _ = std::fs::copy(&old_file, study_data_dir.join(suffix));
        }
    }

    Ok(true)
}

fn write_path_diagnostics(app: &tauri::App, study_data_dir: &Path) {
    let log_path = std::env::temp_dir().join("jizubeikao-path-debug.log");
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map(path_to_string)
        .unwrap_or_else(|e| format!("<error: {e}>"));
    let app_local_data_dir = app
        .path()
        .app_local_data_dir()
        .map(path_to_string)
        .unwrap_or_else(|e| format!("<error: {e}>"));
    let desktop_dir = app
        .path()
        .desktop_dir()
        .map(path_to_string)
        .unwrap_or_else(|e| format!("<error: {e}>"));
    let current_dir = std::env::current_dir()
        .map(path_to_string)
        .unwrap_or_else(|e| format!("<error: {e}>"));

    let content = format!(
        "product_name=计组备考助手\nidentifier=com.fanrenxiuluo.computer-organization-study-assistant\napp_data_dir={app_data_dir}\napp_local_data_dir={app_local_data_dir}\ndesktop_dir={desktop_dir}\ncurrent_dir={current_dir}\nstudy_data_dir={}\ndb_path={}\n",
        study_data_dir.display(),
        study_data_dir.join("study.db").display(),
    );

    let _ = std::fs::write(log_path, content);
}

fn path_to_string(path: PathBuf) -> String {
    path.to_string_lossy().to_string()
}
