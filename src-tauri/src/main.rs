#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod fs_utils;
mod cache;
mod analysis;
mod projects;
mod storage;
mod ai;
mod insights;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            projects::list_project_directories,
            analysis::analyze_repository,
            analysis::analyze_repository_fresh,
            ai::generate_ideas,
            storage::save_settings,
            storage::load_settings,
            ai::load_models,
            ai::generate_project_summary,
            storage::save_project_summary,
            storage::load_project_summary,
            storage::save_root_folder,
            storage::load_root_folder,
            projects::update_project_file_count,
            projects::clear_file_count_cache,
            insights::get_project_insights
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
