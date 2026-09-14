use serde::{Deserialize, Serialize};

const CURRENT_SCHEMA_VERSION: u32 = 1;
const DEFAULT_REFRESH_INTERVAL_SECONDS: u64 = 10;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub schema_version: u32,
    pub scanning: ScanSettings,
    pub adapters: AdapterSettings,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            schema_version: CURRENT_SCHEMA_VERSION,
            scanning: ScanSettings::default(),
            adapters: AdapterSettings::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanSettings {
    pub claude_projects_directory: Option<String>,
    pub codex_home_directory: Option<String>,
    pub refresh_interval_seconds: u64,
}

impl Default for ScanSettings {
    fn default() -> Self {
        Self {
            claude_projects_directory: None,
            codex_home_directory: None,
            refresh_interval_seconds: DEFAULT_REFRESH_INTERVAL_SECONDS,
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdapterSettings {
    pub claude: Option<AdapterCommandOverride>,
    pub codex: Option<AdapterCommandOverride>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdapterCommandOverride {
    pub program: String,
    pub arguments: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::AppSettings;

    #[test]
    fn default_settings_only_select_safe_discovery_defaults() {
        let settings = AppSettings::default();

        assert_eq!(settings.schema_version, 1);
        assert_eq!(settings.scanning.refresh_interval_seconds, 10);
        assert!(settings.scanning.claude_projects_directory.is_none());
        assert!(settings.scanning.codex_home_directory.is_none());
        assert!(settings.adapters.claude.is_none());
        assert!(settings.adapters.codex.is_none());
    }
}
