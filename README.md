# SalatMV

A cli version of the salat mv mobile app. Used to show prayer times in Maldives.

This program uses the db from the salatmv app

![Screenshot_20230316_032642](https://user-images.githubusercontent.com/56493793/225458747-a99470ea-2b04-4f07-9b54-0eeb600d37e8.png)

`~> salat_mv -ct`

```
Salat_MV-cli
---------------------
Time   :  06:34:05 pm
---------------------

Fajr:   05:01 am
Sun:    06:15 am
Dhuhur: 12:24 pm
Asr:    03:44 pm
Magrib: 06:24 pm /------
Isha:   07:39 pm \-
```

# Installation

## Building From Source

## All Platforms (Cargo) (Recommended)

- Install Rust/Cargo on your system

- Run  `cargo install salat_mv`

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

- Run `git checkout [version name]` to checkout to the latest tag. eg `git checkout v0.3.6`

- Run `cargo build --release`

- The executable file will be inside `/target/release/`

# Usage

Run `salat_mv --help` to get the following list of commands:

```css
SalatMV for cli

Usage: salat_mv [option]

Options:
    -h, --help       shows this help section
    -a, --active     keeps the program always running
    -n, --notify     enables notifications when using -a, edits notifications when not using -a (requires 'notify-send' command)
    -e, --edit       edit island index
    -c, --current    indicates the current time
    -t, --title      shows the title bar
    -r, --raw-data   outputs raw data in hours and minutes
    -A, --array      outputs the data in an array
    -m, --minutes    outputs raw data in minutes
    -H, --hour       show time in 24 hour format

config contains island index
config is stored in ~/.config/salat_mv/
```



# Examples:

- You can combine multiple flags like `salat_mv -cantH` (this will run it in always active mode with 24 Hour display while indicating the current time and showing a title bar)

- You can run `salat_mv -A` to get a list of times in an array format like:

    `["04:46 AM", "05:58 AM", "12:09 PM", "03:25 PM", "06:10 PM", "07:24 PM"]`

- You can run `salat_mv -r` to get raw data (without printing all the decorations) like:
  
  ```
  04:46 AM
  05:58 AM
  12:09 PM
  03:25 PM
  06:10 PM
  07:24 PM
  ```

# 



# Todo:

### version 2.0.0

- [ ] Tui mode 

### version 1.0.0

- [x] Notifications via `notify-send`

- [ ] Add notification editing

- [ ] Notification support for windows & mac

- [x] ~~Add notification missed message~~

- [x] Change edit mode to be more usable

- [x] More visual feedback for --current flag

- [x] config storage
