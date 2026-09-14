use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, bail};

pub const FIXTURE_SENTINEL: &str = ".vibestudio-m0-fixture";
const FIXTURE_SENTINEL_CONTENT: &str = "VibeStudio M0 isolated fixture";

pub fn validate_workspace(path: &Path) -> Result<PathBuf> {
    let workspace = dunce::canonicalize(path)
        .with_context(|| format!("cannot resolve M0 workspace {}", path.display()))?;
    if !workspace.is_dir() {
        bail!("M0 workspace is not a directory: {}", workspace.display());
    }

    let sentinel_path = workspace.join(FIXTURE_SENTINEL);
    let sentinel = fs::read_to_string(&sentinel_path).with_context(|| {
        format!(
            "M0 workspace is missing safety sentinel {}",
            sentinel_path.display()
        )
    })?;
    if sentinel.lines().next() != Some(FIXTURE_SENTINEL_CONTENT) {
        bail!(
            "M0 safety sentinel has unexpected content: {}",
            sentinel_path.display()
        );
    }

    Ok(workspace)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::{FIXTURE_SENTINEL, FIXTURE_SENTINEL_CONTENT, validate_workspace};

    #[test]
    fn accepts_marked_fixture() {
        let directory = tempdir().expect("temporary directory should be created");
        fs::write(
            directory.path().join(FIXTURE_SENTINEL),
            format!("{FIXTURE_SENTINEL_CONTENT}\n"),
        )
        .expect("sentinel should be written");

        let validated = validate_workspace(directory.path()).expect("fixture should be accepted");

        assert_eq!(validated, directory.path());
    }

    #[test]
    fn rejects_unmarked_directory() {
        let directory = tempdir().expect("temporary directory should be created");

        let error = validate_workspace(directory.path()).expect_err("fixture should be rejected");

        assert!(error.to_string().contains("missing safety sentinel"));
    }
}
