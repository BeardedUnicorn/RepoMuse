#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod fs_utils;
mod cache;
mod analysis;
mod projects;
mod storage;
mod ai;
mod insights;

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
            projects::clear_file_count_cache,
            insights::get_project_insights,
            insights::get_git_log,
            cache::get_file_metadata_cache_stats,
            cache::get_file_count_cache_stats,
            cache::incremental_update_file_count,
            cache::batch_update_file_counts,
            cache::batch_get_file_metadata,
            cache::batch_update_file_metadata,
            cache::clear_all_caches
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
