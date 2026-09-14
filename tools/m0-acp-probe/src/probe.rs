use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::Duration,
};

use agent_client_protocol::{
    AcpAgent, ActiveSession, Agent, Client, ConnectionTo, Error as AcpError, LineDirection,
    SessionMessage,
    schema::{
        ProtocolVersion,
        v1::{
            CancelNotification, ContentBlock, ContentChunk, InitializeRequest,
            RequestPermissionOutcome, RequestPermissionRequest, RequestPermissionResponse,
            SessionNotification, SessionUpdate, SetSessionConfigOptionRequest, TextContent,
        },
    },
    util::MatchDispatch,
};
use anyhow::Result;
use serde_json::Value;

use crate::{adapter::AdapterSpec, cli::ProbeOperation};

const STDERR_LINE_LIMIT: usize = 100;
const STDERR_CHAR_LIMIT: usize = 2_000;

#[derive(Debug)]
pub struct ProbeOutcome {
    pub session_id: Option<String>,
    pub initialize: Value,
    pub session_setup: Option<Value>,
    pub config_updates: Vec<Value>,
    pub replay_assistant_text: Option<String>,
    pub replay_event_counts: BTreeMap<String, u64>,
    pub replay_events: Vec<Value>,
    pub assistant_text: Option<String>,
    pub stop_reason: Option<String>,
    pub event_counts: BTreeMap<String, u64>,
    pub events: Vec<Value>,
}

#[derive(Clone, Debug, Default)]
pub struct Diagnostics {
    stderr: Arc<Mutex<Vec<String>>>,
    permission_requests: Arc<Mutex<Vec<Value>>>,
}

impl Diagnostics {
    pub fn stderr_tail(&self) -> Vec<String> {
        self.stderr.lock().map_or_else(
            |_| vec!["stderr diagnostics lock was poisoned".to_owned()],
            |lines| lines.clone(),
        )
    }

    pub fn permission_requests(&self) -> Vec<Value> {
        self.permission_requests.lock().map_or_else(
            |_| vec![serde_json::json!({"error": "permission diagnostics lock was poisoned"})],
            |requests| requests.clone(),
        )
    }

    fn record(&self, line: &str, direction: LineDirection) {
        if direction != LineDirection::Stderr {
            return;
        }
        let Ok(mut lines) = self.stderr.lock() else {
            return;
        };
        if lines.len() == STDERR_LINE_LIMIT {
            lines.remove(0);
        }
        lines.push(line.chars().take(STDERR_CHAR_LIMIT).collect());
    }

    fn record_permission(&self, request: &RequestPermissionRequest) -> Result<(), AcpError> {
        let value = serde_json::to_value(request).map_err(AcpError::into_internal_error)?;
        self.permission_requests
            .lock()
            .map_err(|_| AcpError::internal_error())?
            .push(value);
        Ok(())
    }
}

pub async fn execute(
    spec: AdapterSpec,
    operation: ProbeOperation,
    workspace: PathBuf,
    session_config_overrides: Vec<(String, String)>,
    diagnostics: Diagnostics,
) -> Result<ProbeOutcome> {
    let debug_diagnostics = diagnostics.clone();
    let permission_diagnostics = diagnostics.clone();
    let agent = AcpAgent::new(spec.launch_config()).with_debug(move |line, direction| {
        debug_diagnostics.record(line, direction);
    });

    let outcome = Client
        .builder()
        .name("vibestudio-m0-probe")
        .on_receive_request(
            async move |request: RequestPermissionRequest, responder, _connection| {
                permission_diagnostics.record_permission(&request)?;
                responder.respond(RequestPermissionResponse::new(
                    RequestPermissionOutcome::Cancelled,
                ))
            },
            agent_client_protocol::on_receive_request!(),
        )
        .connect_with(agent, async move |connection| {
            let initialize = connection
                .send_request(InitializeRequest::new(ProtocolVersion::V1))
                .block_task()
                .await?;
            let initialize =
                serde_json::to_value(initialize).map_err(AcpError::into_internal_error)?;

            match operation {
                ProbeOperation::Inspect => Ok(inspect_outcome(initialize)),
                ProbeOperation::New { prompt } => {
                    execute_new(
                        &connection,
                        &workspace,
                        &session_config_overrides,
                        &prompt,
                        initialize,
                    )
                    .await
                }
                ProbeOperation::Load { session_id, prompt } => {
                    execute_load(
                        &connection,
                        &workspace,
                        &session_config_overrides,
                        session_id,
                        &prompt,
                        initialize,
                    )
                    .await
                }
                ProbeOperation::Resume { session_id, prompt } => {
                    execute_resume(
                        &connection,
                        &workspace,
                        &session_config_overrides,
                        session_id,
                        &prompt,
                        None,
                        initialize,
                    )
                    .await
                }
                ProbeOperation::Cancel {
                    session_id,
                    prompt,
                    after_ms,
                } => {
                    execute_resume(
                        &connection,
                        &workspace,
                        &session_config_overrides,
                        session_id,
                        &prompt,
                        Some(after_ms),
                        initialize,
                    )
                    .await
                }
            }
        })
        .await?;

    Ok(outcome)
}

fn inspect_outcome(initialize: Value) -> ProbeOutcome {
    ProbeOutcome {
        session_id: None,
        initialize,
        session_setup: None,
        config_updates: Vec::new(),
        replay_assistant_text: None,
        replay_event_counts: BTreeMap::new(),
        replay_events: Vec::new(),
        assistant_text: None,
        stop_reason: None,
        event_counts: BTreeMap::new(),
        events: Vec::new(),
    }
}

async fn execute_new(
    connection: &ConnectionTo<Agent>,
    workspace: &Path,
    config: &[(String, String)],
    prompt: &str,
    initialize: Value,
) -> Result<ProbeOutcome, AcpError> {
    let mut session = connection
        .build_session(workspace)
        .block_task()
        .start_session()
        .await?;
    let session_setup = session_setup(&session)?;
    let config_updates = apply_session_config(&session, config).await?;
    let turn = run_turn(&mut session, prompt).await?;
    session_outcome(
        initialize,
        session_setup,
        config_updates,
        None,
        turn,
        &session,
    )
}

async fn execute_load(
    connection: &ConnectionTo<Agent>,
    workspace: &Path,
    config: &[(String, String)],
    session_id: String,
    prompt: &str,
    initialize: Value,
) -> Result<ProbeOutcome, AcpError> {
    let restored = connection
        .load_session(session_id, workspace)
        .block_task()
        .start_session()
        .await?;
    let session_setup =
        serde_json::to_value(restored.response()).map_err(AcpError::into_internal_error)?;
    let mut session = restored.into_session();
    let replay = drain_replay(&mut session).await?;
    let config_updates = apply_session_config(&session, config).await?;
    let turn = run_turn(&mut session, prompt).await?;
    session_outcome(
        initialize,
        session_setup,
        config_updates,
        Some(replay),
        turn,
        &session,
    )
}

async fn execute_resume(
    connection: &ConnectionTo<Agent>,
    workspace: &Path,
    config: &[(String, String)],
    session_id: String,
    prompt: &str,
    cancel_after_ms: Option<u64>,
    initialize: Value,
) -> Result<ProbeOutcome, AcpError> {
    if cancel_after_ms == Some(0) {
        return Err(AcpError::invalid_params().data("cancel delay must be greater than zero"));
    }
    let restored = connection
        .resume_session(session_id, workspace)
        .block_task()
        .start_session()
        .await?;
    let session_setup =
        serde_json::to_value(restored.response()).map_err(AcpError::into_internal_error)?;
    let mut session = restored.into_session();
    let config_updates = apply_session_config(&session, config).await?;
    let turn = match cancel_after_ms {
        Some(after_ms) => run_cancel_turn(&mut session, prompt, after_ms).await?,
        None => run_turn(&mut session, prompt).await?,
    };
    session_outcome(
        initialize,
        session_setup,
        config_updates,
        None,
        turn,
        &session,
    )
}

fn session_setup(session: &ActiveSession<'static, Agent>) -> Result<Value, AcpError> {
    serde_json::to_value(session.response()).map_err(AcpError::into_internal_error)
}

fn session_outcome(
    initialize: Value,
    session_setup: Value,
    config_updates: Vec<Value>,
    replay: Option<EventCapture>,
    turn: TurnCapture,
    session: &ActiveSession<'static, Agent>,
) -> Result<ProbeOutcome, AcpError> {
    let (replay_assistant_text, replay_event_counts, replay_events) = replay.map_or_else(
        || (None, BTreeMap::new(), Vec::new()),
        |capture| {
            (
                Some(capture.assistant_text),
                capture.event_counts,
                capture.events,
            )
        },
    );
    Ok(ProbeOutcome {
        session_id: Some(session_id_as_string(session.session_id())?),
        initialize,
        session_setup: Some(session_setup),
        config_updates,
        replay_assistant_text,
        replay_event_counts,
        replay_events,
        assistant_text: Some(turn.assistant_text),
        stop_reason: Some(turn.stop_reason),
        event_counts: turn.event_counts,
        events: turn.events,
    })
}

#[derive(Debug)]
struct TurnCapture {
    assistant_text: String,
    stop_reason: String,
    event_counts: BTreeMap<String, u64>,
    events: Vec<Value>,
}

#[derive(Debug, Default)]
struct EventCapture {
    assistant_text: String,
    event_counts: BTreeMap<String, u64>,
    events: Vec<Value>,
}

async fn apply_session_config(
    session: &ActiveSession<'static, Agent>,
    overrides: &[(String, String)],
) -> Result<Vec<Value>, AcpError> {
    let mut updates = Vec::with_capacity(overrides.len());
    for (config_id, value) in overrides {
        let response = session
            .connection()
            .send_request(SetSessionConfigOptionRequest::new(
                session.session_id().clone(),
                config_id.clone(),
                value.as_str(),
            ))
            .block_task()
            .await?;
        updates.push(serde_json::json!({
            "config_id": config_id,
            "requested_value": value,
            "response": serde_json::to_value(response).map_err(AcpError::into_internal_error)?,
        }));
    }
    Ok(updates)
}

async fn run_turn(
    session: &mut ActiveSession<'static, Agent>,
    prompt: &str,
) -> Result<TurnCapture, AcpError> {
    session.send_prompt(prompt)?;
    read_turn(session).await
}

async fn run_cancel_turn(
    session: &mut ActiveSession<'static, Agent>,
    prompt: &str,
    after_ms: u64,
) -> Result<TurnCapture, AcpError> {
    session.send_prompt(prompt)?;
    tokio::time::sleep(Duration::from_millis(after_ms)).await;
    session
        .connection()
        .send_notification(CancelNotification::new(session.session_id().clone()))?;
    read_turn(session).await
}

async fn read_turn(session: &mut ActiveSession<'static, Agent>) -> Result<TurnCapture, AcpError> {
    let mut capture = EventCapture::default();

    loop {
        if let Some(stop_reason) =
            capture_message(session.read_update().await?, &mut capture).await?
        {
            return Ok(TurnCapture {
                assistant_text: capture.assistant_text,
                stop_reason,
                event_counts: capture.event_counts,
                events: capture.events,
            });
        }
    }
}

async fn drain_replay(
    session: &mut ActiveSession<'static, Agent>,
) -> Result<EventCapture, AcpError> {
    let mut capture = EventCapture::default();
    loop {
        match tokio::time::timeout(Duration::from_millis(200), session.read_update()).await {
            Ok(Ok(message)) => {
                if capture_message(message, &mut capture).await?.is_some() {
                    *capture
                        .event_counts
                        .entry("unexpected_replay_stop_reason".to_owned())
                        .or_insert(0) += 1;
                }
            }
            Ok(Err(error)) => return Err(error),
            Err(_) => return Ok(capture),
        }
    }
}

async fn capture_message(
    message: SessionMessage,
    capture: &mut EventCapture,
) -> Result<Option<String>, AcpError> {
    match message {
        SessionMessage::StopReason(reason) => Ok(Some(format!("{reason:?}"))),
        SessionMessage::SessionMessage(dispatch) => {
            let notification_slot = Arc::new(Mutex::new(None));
            let handler_slot = Arc::clone(&notification_slot);
            MatchDispatch::new(dispatch)
                .if_notification(async move |notification: SessionNotification| {
                    let mut slot = handler_slot
                        .lock()
                        .map_err(|_| AcpError::internal_error())?;
                    *slot = Some(notification);
                    Ok(())
                })
                .await
                .otherwise_ignore()?;

            let notification = notification_slot
                .lock()
                .map_err(|_| AcpError::internal_error())?
                .take();
            if let Some(notification) = notification {
                if let SessionUpdate::AgentMessageChunk(ContentChunk {
                    content: ContentBlock::Text(TextContent { text, .. }),
                    ..
                }) = &notification.update
                {
                    capture.assistant_text.push_str(text);
                }
                let event =
                    serde_json::to_value(notification).map_err(AcpError::into_internal_error)?;
                record_event(event, capture);
            }
            Ok(None)
        }
        _ => {
            *capture
                .event_counts
                .entry("unknown_session_message".to_owned())
                .or_insert(0) += 1;
            Ok(None)
        }
    }
}

fn record_event(event: Value, capture: &mut EventCapture) {
    let kind = event
        .pointer("/update/sessionUpdate")
        .and_then(Value::as_str)
        .unwrap_or("unknown")
        .to_owned();
    *capture.event_counts.entry(kind.clone()).or_insert(0) += 1;
    if matches!(
        kind.as_str(),
        "agent_message_chunk" | "agent_thought_chunk" | "user_message_chunk"
    ) {
        return;
    }
    if kind == "available_commands_update" {
        capture.events.retain(|existing| {
            existing
                .pointer("/update/sessionUpdate")
                .and_then(Value::as_str)
                != Some("available_commands_update")
        });
    }
    capture.events.push(event);
}

fn session_id_as_string(session_id: &impl serde::Serialize) -> Result<String, AcpError> {
    serde_json::to_value(session_id)
        .map_err(AcpError::into_internal_error)?
        .as_str()
        .map(ToOwned::to_owned)
        .ok_or_else(|| AcpError::internal_error().data("session ID is not a string"))
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{EventCapture, record_event};

    #[test]
    fn keeps_only_latest_command_snapshot() {
        let mut capture = EventCapture::default();
        record_event(
            json!({"update": {"sessionUpdate": "available_commands_update", "value": 1}}),
            &mut capture,
        );
        record_event(
            json!({"update": {"sessionUpdate": "available_commands_update", "value": 2}}),
            &mut capture,
        );

        assert_eq!(capture.event_counts["available_commands_update"], 2);
        assert_eq!(capture.events.len(), 1);
        assert_eq!(capture.events[0]["update"]["value"], 2);
    }

    #[test]
    fn counts_thought_chunks_without_storing_content() {
        let mut capture = EventCapture::default();
        record_event(
            json!({"update": {"sessionUpdate": "agent_thought_chunk", "content": "private"}}),
            &mut capture,
        );

        assert_eq!(capture.event_counts["agent_thought_chunk"], 1);
        assert!(capture.events.is_empty());
    }
}
