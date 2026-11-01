//! User commands that run when a focus session starts and ends.
//!
//! The commands come from the `[hooks]` table of the config file and run through `sh -c`, so
//! they can call anything, e.g. `curl` to switch do-not-disturb on another device. A failing or
//! hanging hook only prints a warning and never stops the session.

use log::debug;
use rustix::process::{kill_process_group, Pid, Signal};
use serde::Deserialize;
use std::process::Stdio;
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
        if let Err(err) = run(command).await {
            eprintln!("Warning: the {stage} hook `{command}` {err}");
        }
    }
}

/// Runs one command, returning the end of a warning sentence if it fails.
async fn run(command: &str) -> Result<(), String> {
    // Own process group: Ctrl+C in the terminal doesn't reach the hook, and a timeout can
    // kill everything the shell started, not only the shell.
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdin(Stdio::null())
        .process_group(0)
        .kill_on_drop(true)
        .spawn()
        .map_err(|err| format!("can't run: {err}"))?;
    match timeout(HOOK_TIMEOUT, child.wait()).await {
        Ok(Ok(status)) if status.success() => Ok(()),
        Ok(Ok(status)) => Err(format!("failed with {status}")),
        Ok(Err(err)) => Err(format!("failed: {err}")),
        Err(_) => {
            if let Some(pgid) = child.id().and_then(|id| Pid::from_raw(id as i32)) {
                let _ = kill_process_group(pgid, Signal::KILL);
            }
            let _ = child.wait().await;
            Err(format!("took longer than {HOOK_TIMEOUT:?}, killed it"))
        }
    }
}
