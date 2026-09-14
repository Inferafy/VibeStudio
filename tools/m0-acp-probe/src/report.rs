use std::{
    collections::BTreeMap,
    fs,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::{Context, Result};
use serde::Serialize;
use serde_json::Value;

use crate::{adapter::AdapterReport, cli::AgentKind, probe::ProbeOutcome};

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProbeStatus {
    Success,
    Failed,
    TimedOut,
}

#[derive(Debug, Serialize)]
pub struct ProbeReport {
    schema_version: u8,
    observed_at_unix_ms: u64,
    agent: AgentKind,
    operation: String,
    input_session_id: Option<String>,
    workspace: String,
    adapter: AdapterReport,
    status: ProbeStatus,
    session_id: Option<String>,
    initialize: Option<Value>,
    session_setup: Option<Value>,
    config_updates: Vec<Value>,
    replay_assistant_text: Option<String>,
    replay_event_counts: BTreeMap<String, u64>,
    replay_events: Vec<Value>,
    assistant_text: Option<String>,
    stop_reason: Option<String>,
    event_counts: BTreeMap<String, u64>,
    events: Vec<Value>,
    permission_requests: Vec<Value>,
    stderr_tail: Vec<String>,
    error: Option<String>,
}

impl ProbeReport {
    #[allow(clippy::too_many_arguments)]
    pub fn success(
        agent: AgentKind,
        operation: &str,
        input_session_id: Option<String>,
        workspace: &Path,
        adapter: AdapterReport,
        outcome: ProbeOutcome,
        permission_requests: Vec<Value>,
        stderr_tail: Vec<String>,
    ) -> Result<Self> {
        Ok(Self {
            schema_version: 1,
            observed_at_unix_ms: now_unix_ms()?,
            agent,
            operation: operation.to_owned(),
            input_session_id,
            workspace: workspace.to_string_lossy().into_owned(),
            adapter,
            status: ProbeStatus::Success,
            session_id: outcome.session_id,
            initialize: Some(outcome.initialize),
            session_setup: outcome.session_setup,
            config_updates: outcome.config_updates,
            replay_assistant_text: outcome.replay_assistant_text,
            replay_event_counts: outcome.replay_event_counts,
            replay_events: outcome.replay_events,
            assistant_text: outcome.assistant_text,
            stop_reason: outcome.stop_reason,
            event_counts: outcome.event_counts,
            events: outcome.events,
            permission_requests,
            stderr_tail,
            error: None,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn failure(
        agent: AgentKind,
        operation: &str,
        input_session_id: Option<String>,
        workspace: &Path,
        adapter: AdapterReport,
        timed_out: bool,
        error: String,
        permission_requests: Vec<Value>,
        stderr_tail: Vec<String>,
    ) -> Result<Self> {
        Ok(Self {
            schema_version: 1,
            observed_at_unix_ms: now_unix_ms()?,
            agent,
            operation: operation.to_owned(),
            input_session_id,
            workspace: workspace.to_string_lossy().into_owned(),
            adapter,
            status: if timed_out {
                ProbeStatus::TimedOut
            } else {
                ProbeStatus::Failed
            },
            session_id: None,
            initialize: None,
            session_setup: None,
            config_updates: Vec::new(),
            replay_assistant_text: None,
            replay_event_counts: BTreeMap::new(),
            replay_events: Vec::new(),
            assistant_text: None,
            stop_reason: None,
            event_counts: BTreeMap::new(),
            events: Vec::new(),
            permission_requests,
            stderr_tail,
            error: Some(error),
        })
    }

    pub fn write(&self, output: Option<&Path>) -> Result<()> {
        let json = serde_json::to_string_pretty(self).context("cannot serialize probe report")?;
        if let Some(path) = output {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).with_context(|| {
                    format!("cannot create report directory {}", parent.display())
                })?;
            }
            fs::write(path, format!("{json}\n"))
                .with_context(|| format!("cannot write probe report {}", path.display()))?;
        }
        println!("{json}");
        Ok(())
    }
}

fn now_unix_ms() -> Result<u64> {
    let milliseconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("system clock is before Unix epoch")?
        .as_millis();
    u64::try_from(milliseconds).context("current timestamp does not fit in u64")
}
