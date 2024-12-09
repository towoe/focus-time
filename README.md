# Focus Time

Focus Time is a command-line application designed to help manage focus sessions
effectively. It allows to set a time in which notifications are disabled. It
integrates with [Sway](https://github.com/swaywm/sway/) and
[SwayNC](https://github.com/ErikReider/SwayNotificationCenter/).

## Installation

To install Focus Time, you need to have Rust and Cargo installed on your
system, see the [Rust docs](https://www.rust-lang.org/tools/install).

Clone the repository and build the project:

```console
$ git clone https://github.com/towoe/focus-time.git
$ cd focus-time
$ cargo build --release
```

## Usage

To use Focus Time, run the following command to start the default focus time of
25 minutes, hide the Sway status bar and send a notification at the end of the
time:

```sh
$ focus-time
```

Set a custom time by specifying the duration with a number and unit (e.g. `5s`,
`10m`, `1h`):

```console
$ focus-time 20m
```

For a full list of available options, use:

```sh
$ focus-time --help
```

### Client access

When the application is running, it can be accessed via D-Bus, to retrieve the
remaining time and to stop it.

For example, `busctl` can be used to print the remaining time:

```
$ busctl --user call org.towoe.FocusTime /org/towoe/FocusTime \
org.towoe.FocusTime GetRemainingTime
```

### Integration

To integrate Focus Time with Sway, you can bind keys to start a 25-minute focus
session and stop the timer using the following configuration, typically located
at `~/.config/sway/config`:

```ini
# Focus time
bindsym $mod+o exec focus-time 25m
bindsym $mod+Shift+o exec busctl --user call org.towoe.FocusTime \
/org/towoe/FocusTime org.towoe.FocusTime StopTimer
```
