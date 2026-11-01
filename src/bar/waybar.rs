//! Waybar, toggled with `SIGUSR1` for compositors without a bar protocol, like niri.
//!
//! Waybar can't report whether it is visible, so hiding assumes it is shown and restoring
//! toggles the same processes back.

use std::fs;

use anyhow::Result;
use log::debug;
use rustix::process::{kill_process, Pid, Signal};

pub struct Waybar {
    /// Processes toggled by [`Waybar::hide`]; a Waybar started later is left alone.
    hidden: Vec<Pid>,
}

impl Waybar {
    /// Returns `None` if no Waybar is running.
    pub fn find() -> Option<Self> {
        let running = waybar_pids();
        (!running.is_empty()).then_some(Self { hidden: Vec::new() })
    }

    pub fn hide(&mut self) -> Result<()> {
        self.hidden = waybar_pids();
        debug!("Hiding Waybar: {:?}", self.hidden);
        toggle(&self.hidden)
    }

    pub fn restore(&mut self) -> Result<()> {
        // A PID that no longer belongs to Waybar was restarted or reused.
        let pids: Vec<_> = self
            .hidden
            .drain(..)
            .filter(|&pid| is_waybar(pid))
            .collect();
        debug!("Showing Waybar: {pids:?}");
        toggle(&pids)
    }
}

fn toggle(pids: &[Pid]) -> Result<()> {
    for &pid in pids {
        kill_process(pid, Signal::USR1)?;
    }
    Ok(())
}

fn waybar_pids() -> Vec<Pid> {
    let Ok(entries) = fs::read_dir("/proc") else {
        return Vec::new();
    };
    entries
        .filter_map(|entry| entry.ok()?.file_name().to_str()?.parse().ok())
        .filter_map(Pid::from_raw)
        .filter(|&pid| is_waybar(pid))
        .collect()
}

fn is_waybar(pid: Pid) -> bool {
    fs::read_to_string(format!("/proc/{}/comm", pid.as_raw_nonzero()))
        .is_ok_and(|comm| comm.trim_end() == "waybar")
}
