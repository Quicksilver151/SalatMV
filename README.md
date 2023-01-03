# SalatMV

A cli version of salat mv.

`~> salat_mv -t`

```
Salat_MV-cli
---------------------
Time   :  10:07 pm
Island :  WIP
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

## Building From Source

### Linux

- install cargo on your system
- run `build_for_linux.sh`
- a file link should be available in ./target

# Usage

run `salat_mv --help` to get the following list of commands:

```css
SalatMV for cli

Usage: salat_mv [option]

Options:
    -h, --help       shows this help section
    -T, --tui        runs in tui mode (not implemented yet)
    -n, --notify     runs in notify mode (requires \'notify-send\' command)
    -e, --edit       edit island index
    -c, --current    indicates the current time
    -t, --title      shows the title bar
    -r, --raw-data   outputs raw data in hours and minutes
    -m, --minutes    outputs raw data in minutes
    -H, --hour       show time in 24 hour format

config contains island index
config is stored in ~/.config/salat_mv/
```

you can combine multiple flags like `salat_mv -ctH`

# Todo:

### version 2.0.0

- [ ] Tui mode 

### version 1.0.0

- [ ] Notifications via `notify-send`

- [ ] Change edit mode to be more usable

- [ ] More visual feedback for --current flag

- [x] config storage
