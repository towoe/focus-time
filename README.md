# Focus Time

Focus Time is a command-line application designed to help manage focus sessions
effectively. It allows to set a time in which notifications are disabled. It
integrates with [Sway](https://github.com/swaywm/sway/) and
[SwayNC](https://github.com/ErikReider/SwayNotificationCenter/).

## Installation

To install Focus Time, you need to have Rust and Cargo installed on your
system, see the [Rust docs](https://www.rust-lang.org/tools/install).

Clone the repository and build the project:

```sh
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

Set a custom time:

```sh
$ focus-time 20m
```

To print the remaining time:

```sh
$ focus-time -p
```

For a full list of available options, use:

```sh
$ focus-time --help
```
