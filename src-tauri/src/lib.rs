pub mod acp;
pub mod scanner;
pub mod settings;

mod commands;
mod state;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(state::AppState::default())
        .invoke_handler(tauri::generate_handler![commands::get_app_snapshot])
        .run(tauri::generate_context!())
        .expect("error while running VibeStudio");
}
