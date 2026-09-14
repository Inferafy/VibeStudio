#![deny(warnings)]
#![forbid(unsafe_code)]

mod adapter;
mod cli;
mod fixture;
mod probe;
mod report;

use std::{path::PathBuf, time::Duration};

use anyhow::{Result, bail};
use clap::Parser;

use crate::{
    adapter::AdapterSpec, cli::Cli, fixture::validate_workspace, probe::Diagnostics,
    report::ProbeReport,
};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    if cli.timeout_seconds == 0 {
        bail!("timeout must be greater than zero");
    }

    let workspace = validate_workspace(&cli.workspace)?;
    let session_config_overrides = cli.session_config_overrides();
    let adapter_root = cli
        .adapter_root
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")));
    let adapter = AdapterSpec::resolve(cli.agent, &adapter_root, &cli.node)?;
    let adapter_report = adapter.report();
    let operation_name = cli.operation.name();
    let input_session_id = cli.operation.input_session_id().map(ToOwned::to_owned);
    let diagnostics = Diagnostics::default();

    let execution = tokio::time::timeout(
        Duration::from_secs(cli.timeout_seconds),
        probe::execute(
            adapter,
            cli.operation,
            workspace.clone(),
            session_config_overrides,
            diagnostics.clone(),
        ),
    )
    .await;

    let (report, failed) = match execution {
        Ok(Ok(outcome)) => (
            ProbeReport::success(
                cli.agent,
                operation_name,
                input_session_id,
                &workspace,
                adapter_report,
                outcome,
                diagnostics.permission_requests(),
                diagnostics.stderr_tail(),
            )?,
            false,
        ),
        Ok(Err(error)) => (
            ProbeReport::failure(
                cli.agent,
                operation_name,
                input_session_id,
                &workspace,
                adapter_report,
                false,
                format!("{error:#}"),
                diagnostics.permission_requests(),
                diagnostics.stderr_tail(),
            )?,
            true,
        ),
        Err(_) => (
            ProbeReport::failure(
                cli.agent,
                operation_name,
                input_session_id,
                &workspace,
                adapter_report,
                true,
                format!("probe exceeded {} seconds", cli.timeout_seconds),
                diagnostics.permission_requests(),
                diagnostics.stderr_tail(),
            )?,
            true,
        ),
    };

    report.write(cli.output.as_deref())?;
    if failed {
        bail!("M0 probe failed; inspect the JSON report");
    }
    Ok(())
}
