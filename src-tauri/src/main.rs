#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod fs_utils;
mod db;
mod analysis;
mod projects;
mod storage;
mod ai;
mod insights;

use tauri::Manager;
use std::sync::Arc;

fn main() {
    // Tune rayon global thread pool to a sensible cap
    let threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
        .min(8);
    let _ = rayon::ThreadPoolBuilder::new().num_threads(threads).build_global();

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Initialize database
            let app_dir = dirs::data_local_dir()
                .ok_or("Failed to get app data directory")?
                .join("repomuse");
            
            if !app_dir.exists() {
                std::fs::create_dir_all(&app_dir)
                    .map_err(|e| format!("Failed to create app directory: {}", e))?;
            }
            
            let db_path = app_dir.join("repomuse.db");
            let db_pool = db::init_db_pool(&db_path)
                .map_err(|e| format!("Failed to initialize database: {}", e))?;
            
            // Store database pool in app state
            app.manage(Arc::new(db_pool));
            
            // Maximize the main window on startup (not fullscreen)
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.maximize();
            }
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            projects::list_project_directories,
            analysis::analyze_repository,
            analysis::analyze_repository_fresh,
            analysis::analyze_repository_lazy,
            analysis::trigger_full_scan,
            analysis::cancel_analysis,
            analysis::analyze_multiple_repositories,
            ai::generate_ideas,
            storage::save_settings,
            storage::load_settings,
            ai::load_models,
            ai::generate_project_summary,
            storage::save_theme_preference,
            storage::load_theme_preference,
            storage::save_project_summary,
            storage::load_project_summary,
            storage::save_root_folder,
            storage::load_root_folder,
            storage::save_task_list,
            storage::load_task_list,
            storage::save_favorite_projects,
            storage::load_favorite_projects,
            projects::update_project_file_count,
            insights::get_project_insights,
            insights::get_git_log,
            storage::clear_all_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}