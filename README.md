# SalatMV

A cli version of the salat mv mobile app. Used to show prayer times in Maldives.

This program uses the db from the salatmv app



`~> salat_mv -t`

```
Salat_MV-cli
---------------------
Time   :  10:07:01 pm
---------------------

Fajr:   04:50 am
Sun:    06:07 am
Dhuhur: 12:15 pm
Asr:    03:37 pm
Magrib: 06:16 pm
Isha:   07:33 pm
```

# Installation

## Release Binaries

You can find the releases in the [releases](https://github.com/Quicksilver151/SalatMV/releases) tab

only binaries for linux is available

## Building From Source

### Linux (with the shellscript)

- Install Rust/Cargo on your system
- Run `git tag -l` to get a list of tags
- Run `git checkout [version]` to checkout to the latest tag (regular commits are not stable)
- Run `build_for_linux.sh`
- A file link should be available in ./target

### All Platforms (manually)

- Install Git to on your system

- Install Rust/Cargo on your system

- Clone this repository with `git clone https://github.com/Quicksilver151/SalatMV.git`

- Open the folder inside a terminal

- Run `git tag -l` to get a list of tags

- Run `git checkout [version name]` to checkout to the latest tag. eg `git checkout v0.3.1`

- Run `cargo build --release`

- Copy the csv files from `/src/` to `/target/release/`

- The executable file will be inside `/target/release/`



# Usage

Run `salat_mv --help` to get the following list of commands:

```css
SalatMV for cli

Usage: salat_mv [option]

Options:
    -h, --help       shows this help section
    -T, --tui        runs in tui mode (not implemented yet)
    -a, --active     keeps the program always running
    -n, --notify     enables notifications when using -a, edits notifications when not using -a (requires 'notify-send' command)
    -e, --edit       edit island index
    -c, --current    indicates the current time
    -t, --title      shows the title bar
    -r, --raw-data   outputs raw data in hours and minutes
    -m, --minutes    outputs raw data in minutes
    -H, --hour       show time in 24 hour format

config contains island index
config is stored in ~/.config/salat_mv/
```

You can combine multiple flags like `salat_mv -ctH`

# Todo:

### version 2.0.0

- [ ] Tui mode 

### version 1.0.0

- [x] Notifications via `notify-send`

- [ ] Add notification editing

- [ ] Add notification missed message

- [ ] Change edit mode to be more usable

- [x] More visual feedback for --current flag

- [x] config storage
