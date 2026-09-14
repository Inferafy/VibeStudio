use tauri::State;

use crate::state::{AppSnapshot, AppState};

#[tauri::command]
pub(crate) fn get_app_snapshot(state: State<'_, AppState>) -> Result<AppSnapshot, String> {
    state.snapshot().map_err(str::to_owned)
}
