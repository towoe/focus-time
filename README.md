# Focus Time

Focus Time is a command-line application designed to help manage focus sessions
effectively. It allows you to set a time in which notifications are disabled and
the status bar is hidden. It pauses notifications in
[SwayNC](https://github.com/ErikReider/SwayNotificationCenter/) or
[dunst](https://dunst-project.org/), whichever is running. When the session
ends, do-not-disturb returns to the state it had before.

The status bar is hidden through [Sway](https://github.com/swaywm/sway/) when it
runs, and otherwise through [Waybar](https://github.com/Alexays/Waybar) directly,
e.g. under [niri](https://github.com/YaLTeR/niri). Waybar is toggled with
`SIGUSR1`, so it has to be visible when the session starts and keep the default
`on-sigusr1` action, `toggle`. Without Sway or a running Waybar, the session runs
with a warning and leaves the bar alone.

## Installation

To install Focus Time, you need to have Rust and Cargo installed on your
system, see the [Rust docs](https://www.rust-lang.org/tools/install).

Clone the repository and build the project:

```sh
$ git clone https://github.com/towoe/focus-time.git
$ cd focus-time
$ cargo install --path .
```

The binary will be installed to `~/.cargo/bin/`. Make sure this directory is in
your `PATH` environment variable.

## Usage

To use Focus Time, run the following command to start the default focus time of
25 minutes, hide the Sway status bar and send a notification at the end of the
time:

```sh
$ focus-time
```

Set a custom time by specifying the duration with a number and unit (e.g. `5s`,
`10m`, `1h`):

```sh
$ focus-time 20m
```

To change the default behaviour the following options are available:
- `-c, --config`: Path to configuration file
- `-l, --log-level`: Log level (error, warn, info, debug, trace)
- `-n, --no-notification`: Disable timer-end notification
- `-p, --print-time`: Print the remaining time continuously
- `-s, --keep-status-bar`: Keep the status bar visible

## Configuration

Focus Time can be configured using a TOML file. The default location is 
`~/.config/focus-time/config.toml`, or specify a custom path with `-c`.

Example configuration:
```toml
duration = "30m"
no-notification = false
keep-status-bar = false
print-time = false
```

### Hooks

Commands in the `[hooks]` table run through `sh -c` when a session starts and
when it ends, also when it is stopped early. They run one after another, each
may take up to 10 seconds, and a failing command only prints a warning. The
start commands run while the timer already counts down; the end commands wait
for them to finish.

For example, to switch do-not-disturb on an Android phone through a
[MacroDroid](https://www.macrodroid.com/) webhook trigger:

```toml
[hooks]
start = ["curl -fsS 'https://trigger.macrodroid.com/<device-id>/dnd?state=true'"]
end = ["curl -fsS 'https://trigger.macrodroid.com/<device-id>/dnd?state=false'"]
```

## Controlling a running instance

When the application is running, you can control it using subcommands. This is
achieved by communicating with the running instance over its D-Bus interface.

- `focus-time status`: Show the remaining time and pause state.
- `focus-time toggle-pause`: Toggle the pause state of the timer.
- `focus-time stop`: Stop the timer.

Example:
```sh
$ focus-time status
14:32

$ focus-time toggle-pause
Focus timer toggled pause.

$ focus-time status
14:32 (paused)

$ focus-time stop
Focus timer stopped.
```

## Integration

To integrate Focus Time with Sway, you can bind keys to start a 25-minute focus
session and stop the timer using the following configuration.

```ini
# Focus time
bindsym $mod+o exec focus-time 25m
bindsym $mod+Shift+o exec focus-time stop
```

The same for niri:

```kdl
binds {
    Mod+O { spawn "focus-time" "25m"; }
    Mod+Shift+O { spawn "focus-time" "stop"; }
}
```

## Shell Completion

`focus-time` can generate completion scripts for various shells. Use the
`completions` subcommand to generate a script for your shell.

For example, to generate completions for Zsh, you can run:

```sh
$ focus-time completions zsh > _focus-time
```

Store the file `_focus-time` in the appropriate location.
