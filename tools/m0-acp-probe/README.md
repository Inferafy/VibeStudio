# VibeStudio M0 ACP Probe

This tool verifies ACP initialization, session creation, session loading or resuming, prompt delivery, and process cleanup against pinned Claude Code and Codex adapters.

It is intentionally separate from the future Tauri application. Results from this probe establish the compatibility boundary that the production client will implement.

## Safety boundary

The workspace passed to the probe must contain a file named `.vibestudio-m0-fixture` whose first line is:

```text
VibeStudio M0 isolated fixture
```

The probe cancels every ACP permission request. It does not expose file or terminal callbacks to the agent. Claude authentication through a claude.ai subscription is hidden; Claude validation therefore uses an approved Console/API, cloud-provider, or gateway credential.

To keep verification costs bounded, sessions default to Codex `gpt-5.6-luna` or Claude `sonnet` with low effort. Use `--model`, `--effort`, or `--preserve-session-config` only when a specific compatibility case requires different settings.

## Install

```powershell
pnpm --dir tools/m0-acp-probe install --frozen-lockfile
cargo build --manifest-path tools/m0-acp-probe/Cargo.toml
```

Run `pnpm install` without `--frozen-lockfile` only when intentionally updating the pinned adapter lockfile.

## Commands

Initialize an adapter and record its advertised capabilities:

```powershell
cargo run --manifest-path tools/m0-acp-probe/Cargo.toml -- `
  --agent codex `
  --workspace C:\code\VibeStudio-M0-Fixture `
  inspect
```

Create a session and send the first prompt:

```powershell
cargo run --manifest-path tools/m0-acp-probe/Cargo.toml -- `
  --agent codex `
  --workspace C:\code\VibeStudio-M0-Fixture `
  new --prompt "Remember marker M0-CODEX-EXAMPLE and reply with RECORDED only."
```

Resume the returned native session ID in a fresh adapter process:

```powershell
cargo run --manifest-path tools/m0-acp-probe/Cargo.toml -- `
  --agent codex `
  --workspace C:\code\VibeStudio-M0-Fixture `
  resume --session-id SESSION_ID --prompt "Reply with the marker from the previous turn only."
```

Use `load` instead of `resume` when history replay is part of the check. JSON is printed to stdout. Pass `--output .m0-work/<name>.json` to retain a local raw result; `.m0-work/` is ignored by Git.

Raw reports can include native session IDs, workspace paths, command metadata, and user-level command or skill names. Keep them under `.m0-work/`; commit only a redacted validation summary.

Cancel a running turn after a bounded delay:

```powershell
cargo run --manifest-path tools/m0-acp-probe/Cargo.toml -- `
  --agent codex `
  --workspace C:\code\VibeStudio-M0-Fixture `
  cancel --session-id SESSION_ID --after-ms 500 --prompt "Write the integers 1 through 10000."
```
