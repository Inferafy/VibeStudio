use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize, ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum AgentKind {
    Claude,
    Codex,
}

#[derive(Clone, Debug, Subcommand)]
pub enum ProbeOperation {
    /// Initialize the adapter and report its advertised capabilities.
    Inspect,
    /// Create a new native session and send one prompt.
    New {
        #[arg(long)]
        prompt: String,
    },
    /// Load a native session with history replay, then send one prompt.
    Load {
        #[arg(long)]
        session_id: String,
        #[arg(long)]
        prompt: String,
    },
    /// Resume a native session without history replay, then send one prompt.
    Resume {
        #[arg(long)]
        session_id: String,
        #[arg(long)]
        prompt: String,
    },
    /// Resume a session, send a prompt, then cancel the running turn.
    Cancel {
        #[arg(long)]
        session_id: String,
        #[arg(long)]
        prompt: String,
        #[arg(long, default_value_t = 500)]
        after_ms: u64,
    },
}

impl ProbeOperation {
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Inspect => "inspect",
            Self::New { .. } => "new",
            Self::Load { .. } => "load",
            Self::Resume { .. } => "resume",
            Self::Cancel { .. } => "cancel",
        }
    }

    pub fn input_session_id(&self) -> Option<&str> {
        match self {
            Self::Load { session_id, .. }
            | Self::Resume { session_id, .. }
            | Self::Cancel { session_id, .. } => Some(session_id),
            Self::Inspect | Self::New { .. } => None,
        }
    }
}

#[derive(Debug, Parser)]
#[command(name = "vibestudio-m0-acp-probe")]
#[command(about = "Verify VibeStudio ACP session compatibility")]
pub struct Cli {
    #[arg(long, value_enum)]
    pub agent: AgentKind,

    #[arg(long)]
    pub workspace: PathBuf,

    #[arg(long, default_value = "node")]
    pub node: PathBuf,

    #[arg(long)]
    pub adapter_root: Option<PathBuf>,

    #[arg(long, default_value_t = 240)]
    pub timeout_seconds: u64,

    #[arg(long)]
    pub model: Option<String>,

    #[arg(long)]
    pub effort: Option<String>,

    #[arg(long)]
    pub preserve_session_config: bool,

    #[arg(long)]
    pub output: Option<PathBuf>,

    #[command(subcommand)]
    pub operation: ProbeOperation,
}

impl Cli {
    pub fn session_config_overrides(&self) -> Vec<(String, String)> {
        if self.preserve_session_config {
            return Vec::new();
        }

        let model = self.model.clone().unwrap_or_else(|| match self.agent {
            AgentKind::Claude => "sonnet".to_owned(),
            AgentKind::Codex => "gpt-5.6-luna".to_owned(),
        });
        let effort = self.effort.clone().unwrap_or_else(|| "low".to_owned());
        let effort_id = match self.agent {
            AgentKind::Claude => "effort",
            AgentKind::Codex => "reasoning_effort",
        };

        vec![("model".to_owned(), model), (effort_id.to_owned(), effort)]
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::{AgentKind, Cli, ProbeOperation};

    fn cli(agent: AgentKind, preserve_session_config: bool) -> Cli {
        Cli {
            agent,
            workspace: PathBuf::from("fixture"),
            node: PathBuf::from("node"),
            adapter_root: None,
            timeout_seconds: 240,
            model: None,
            effort: None,
            preserve_session_config,
            output: None,
            operation: ProbeOperation::Inspect,
        }
    }

    #[test]
    fn defaults_claude_to_sonnet_low() {
        assert_eq!(
            cli(AgentKind::Claude, false).session_config_overrides(),
            vec![
                ("model".to_owned(), "sonnet".to_owned()),
                ("effort".to_owned(), "low".to_owned()),
            ]
        );
    }

    #[test]
    fn defaults_codex_to_luna_low() {
        assert_eq!(
            cli(AgentKind::Codex, false).session_config_overrides(),
            vec![
                ("model".to_owned(), "gpt-5.6-luna".to_owned()),
                ("reasoning_effort".to_owned(), "low".to_owned()),
            ]
        );
    }

    #[test]
    fn can_preserve_existing_session_config() {
        assert!(
            cli(AgentKind::Claude, true)
                .session_config_overrides()
                .is_empty()
        );
    }
}
