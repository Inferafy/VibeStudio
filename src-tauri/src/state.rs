use std::sync::{Arc, RwLock};

use serde::Serialize;

use crate::{acp::runtime::AcpRuntime, settings::AppSettings};

#[derive(Debug)]
pub struct AppState {
    settings: RwLock<AppSettings>,
    acp_runtime: Arc<AcpRuntime>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            settings: RwLock::new(AppSettings::default()),
            acp_runtime: Arc::new(AcpRuntime::default()),
        }
    }
}

impl AppState {
    pub fn snapshot(&self) -> Result<AppSnapshot, &'static str> {
        let settings = self
            .settings
            .read()
            .map_err(|_| "application settings lock is poisoned")?
            .clone();

        Ok(AppSnapshot {
            settings,
            active_connection_count: self.acp_runtime.active_connection_count(),
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSnapshot {
    pub settings: AppSettings,
    pub active_connection_count: usize,
}

#[cfg(test)]
mod tests {
    use super::AppState;

    #[test]
    fn default_snapshot_uses_one_shared_runtime_and_safe_settings() {
        let snapshot = AppState::default()
            .snapshot()
            .expect("default application state should be readable");

        assert_eq!(snapshot.active_connection_count, 0);
        assert_eq!(snapshot.settings.schema_version, 1);
        assert_eq!(snapshot.settings.scanning.refresh_interval_seconds, 10);
    }
}
