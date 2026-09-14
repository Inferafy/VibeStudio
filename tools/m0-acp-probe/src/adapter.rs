use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use agent_client_protocol::AcpAgentConfig;
use anyhow::{Context, Result, bail};
use serde::Serialize;

use crate::cli::AgentKind;

const CLAUDE_PACKAGE: &str = "@agentclientprotocol/claude-agent-acp";
const CLAUDE_VERSION: &str = "0.77.0";
const CODEX_PACKAGE: &str = "@agentclientprotocol/codex-acp";
const CODEX_VERSION: &str = "1.11.0";

#[derive(Clone, Debug, Serialize)]
pub struct AdapterReport {
    pub package: &'static str,
    pub version: &'static str,
    pub node: String,
    pub entry: String,
    pub arguments: Vec<String>,
    pub environment_names: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct AdapterSpec {
    node: PathBuf,
    entry: PathBuf,
    package: &'static str,
    version: &'static str,
    arguments: Vec<String>,
    environment: BTreeMap<String, String>,
}

impl AdapterSpec {
    pub fn resolve(agent: AgentKind, adapter_root: &Path, node: &Path) -> Result<Self> {
        let node = which::which(node)
            .with_context(|| format!("cannot resolve Node executable {}", node.display()))?;
        let (package, version, relative_entry, arguments, environment) = match agent {
            AgentKind::Claude => (
                CLAUDE_PACKAGE,
                CLAUDE_VERSION,
                Path::new("node_modules")
                    .join("@agentclientprotocol")
                    .join("claude-agent-acp")
                    .join("dist")
                    .join("index.js"),
                vec!["--hide-claude-auth".to_owned()],
                BTreeMap::new(),
            ),
            AgentKind::Codex => (
                CODEX_PACKAGE,
                CODEX_VERSION,
                Path::new("node_modules")
                    .join("@agentclientprotocol")
                    .join("codex-acp")
                    .join("dist")
                    .join("index.js"),
                Vec::new(),
                BTreeMap::from([
                    ("INITIAL_AGENT_MODE".to_owned(), "read-only".to_owned()),
                    ("NO_BROWSER".to_owned(), "1".to_owned()),
                ]),
            ),
        };
        let entry = dunce::canonicalize(adapter_root.join(relative_entry)).with_context(|| {
            format!(
                "adapter {package}@{version} is not installed under {}",
                adapter_root.display()
            )
        })?;
        if !entry.is_file() {
            bail!("adapter entry is not a file: {}", entry.display());
        }

        Ok(Self {
            node,
            entry,
            package,
            version,
            arguments,
            environment,
        })
    }

    pub fn launch_config(&self) -> AcpAgentConfig {
        AcpAgentConfig::new(&self.node)
            .arg(self.entry.to_string_lossy().into_owned())
            .args(self.arguments.clone())
            .envs(self.environment.clone())
    }

    pub fn report(&self) -> AdapterReport {
        AdapterReport {
            package: self.package,
            version: self.version,
            node: self.node.to_string_lossy().into_owned(),
            entry: self.entry.to_string_lossy().into_owned(),
            arguments: self.arguments.clone(),
            environment_names: self.environment.keys().cloned().collect(),
        }
    }
}
