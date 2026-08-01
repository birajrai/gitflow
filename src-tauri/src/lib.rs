mod commands;
mod types;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::git::open_repo,
            commands::git::get_status,
            commands::git::get_diff,
            commands::git::stage_all,
            commands::git::commit,
            commands::git::push,
            commands::git::reset_soft,
            commands::nim::generate_commit_message,
            commands::nim::test_nim_connection,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
