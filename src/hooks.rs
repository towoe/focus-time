//! User commands that run when a focus session starts and ends.
//!
//! The commands come from the `[hooks]` table of the config file and run through `sh -c`, so
//! they can call anything, e.g. `curl` to switch do-not-disturb on another device. A failing or
//! hanging hook only prints a warning and never stops the session.

use log::debug;
use serde::Deserialize;
use std::time::Duration;
use tokio::process::Command;
use tokio::time::timeout;

/// How long a single hook may run before it is killed.
const HOOK_TIMEOUT: Duration = Duration::from_secs(10);

/// Commands to run at the session boundaries.
#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Hooks {
    /// Commands run after the session has started.
    #[serde(default)]
    pub start: Vec<String>,
    /// Commands run after the session has ended, including when it was aborted.
    #[serde(default)]
    pub end: Vec<String>,
}

impl Hooks {
    /// Runs the start commands.
    pub async fn run_start(&self) {
        run_all("start", &self.start).await;
    }

    /// Runs the end commands.
    pub async fn run_end(&self) {
        run_all("end", &self.end).await;
    }
}

/// Runs each command in order and waits for it, so a later command can rely on an earlier one.
async fn run_all(stage: &str, commands: &[String]) {
    for command in commands {
        debug!("Running {stage} hook `{command}`");
        let status = Command::new("sh")
            .arg("-c")
            .arg(command)
            .kill_on_drop(true)
            .status();
        match timeout(HOOK_TIMEOUT, status).await {
            Ok(Ok(status)) if status.success() => {}
            Ok(Ok(status)) => {
                eprintln!("Warning: the {stage} hook `{command}` failed with {status}")
            }
            Ok(Err(err)) => eprintln!("Warning: can't run the {stage} hook `{command}`: {err}"),
            Err(_) => eprintln!(
                "Warning: the {stage} hook `{command}` took longer than {HOOK_TIMEOUT:?}, killed it"
            ),
        }
    }
}
