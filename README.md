# Focus Time

Focus Time is a command-line application designed to help manage focus sessions
effectively. It allows you to set a time in which notifications are disabled. It
integrates with [Sway](https://github.com/swaywm/sway/) and
[SwayNC](https://github.com/ErikReider/SwayNotificationCenter/).

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

## Client access

When the application is running, it can be accessed via D-Bus, to retrieve the
remaining time and to stop it.

For example, `busctl` can be used to print the remaining time:

```sh
$ busctl --user call org.towoe.FocusTime /org/towoe/FocusTime \
  org.towoe.FocusTime GetRemainingTime
```

```sh
$ busctl --user call org.towoe.FocusTime /org/towoe/FocusTime \
  org.towoe.FocusTime TogglePause
```

## Integration

To integrate Focus Time with Sway, you can bind keys to start a 25-minute focus
session and stop the timer using the following configuration.

```ini
# Focus time
bindsym $mod+o exec focus-time 25m
bindsym $mod+Shift+o exec busctl --user call org.towoe.FocusTime \
/org/towoe/FocusTime org.towoe.FocusTime StopTimer
```
