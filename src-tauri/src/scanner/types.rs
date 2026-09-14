use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SessionProvider {
    Claude,
    Codex,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ProviderAvailability {
    Available,
    Partial,
    Unavailable,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionSummary {
    pub provider: SessionProvider,
    pub session_id: String,
    pub title: String,
    pub working_directory: String,
    pub source_path: String,
    pub last_activity_at: i64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectSummary {
    pub project_key: String,
    pub display_name: String,
    pub working_directory: String,
    pub last_activity_at: i64,
    pub sessions: Vec<SessionSummary>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderReport {
    pub provider: SessionProvider,
    pub availability: ProviderAvailability,
    pub session_count: usize,
    pub warnings: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionCatalog {
    pub projects: Vec<ProjectSummary>,
    pub providers: Vec<ProviderReport>,
    pub scanned_at: i64,
}

#[cfg(test)]
mod tests {
    use super::{
        ProjectSummary, ProviderAvailability, ProviderReport, SessionCatalog, SessionProvider,
        SessionSummary,
    };

    #[test]
    fn matches_the_shared_frontend_contract_fixture() {
        let project_directory = r"C:\code\fixture".to_owned();
        let catalog = SessionCatalog {
            projects: vec![ProjectSummary {
                project_key: "c:/code/fixture".to_owned(),
                display_name: "fixture".to_owned(),
                working_directory: project_directory.clone(),
                last_activity_at: 1_789_411_200_000,
                sessions: vec![
                    SessionSummary {
                        provider: SessionProvider::Codex,
                        session_id: "codex-session".to_owned(),
                        title: "Codex fixture".to_owned(),
                        working_directory: project_directory.clone(),
                        source_path: r"C:\Users\fixture\.codex\state.sqlite".to_owned(),
                        last_activity_at: 1_789_411_200_000,
                    },
                    SessionSummary {
                        provider: SessionProvider::Claude,
                        session_id: "claude-session".to_owned(),
                        title: "Claude fixture".to_owned(),
                        working_directory: project_directory,
                        source_path: r"C:\Users\fixture\.claude\projects\session.jsonl".to_owned(),
                        last_activity_at: 1_789_411_140_000,
                    },
                ],
            }],
            providers: vec![
                ProviderReport {
                    provider: SessionProvider::Codex,
                    availability: ProviderAvailability::Available,
                    session_count: 1,
                    warnings: Vec::new(),
                },
                ProviderReport {
                    provider: SessionProvider::Claude,
                    availability: ProviderAvailability::Partial,
                    session_count: 1,
                    warnings: vec!["one damaged record".to_owned()],
                },
            ],
            scanned_at: 1_789_411_260_000,
        };
        let fixture = include_str!("../../../src/sessions/fixtures/session-catalog.json");
        let expected: serde_json::Value =
            serde_json::from_str(fixture).expect("shared fixture should be valid JSON");

        assert_eq!(
            serde_json::to_value(&catalog).expect("session catalog should serialize"),
            expected
        );
        assert_eq!(
            serde_json::from_str::<SessionCatalog>(fixture)
                .expect("shared fixture should deserialize into the Rust contract"),
            catalog
        );
    }
}
