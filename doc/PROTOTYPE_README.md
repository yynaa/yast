# YAST Pre-built Prototype Notice

This notice is for installing a pre-built version of the YAST prototype.

## Contents

- YAST-[xx.xxxx-xxxx]-[target].zip
  - *yast(.exe)* - YAST's main app
  - *yasle(.exe)* - YAST's layout editor (may be unstable)
  - *components/* - default YAST components, need to be installed in data dir
  - *lib/* - default YAST libs, need to be installed in data dir
  - *layouts/basic.yasl* - a default YAST layout example!

## Installation

- Extract contents anywhere you'd like,
- Copy *components/* and *lib/* in your data directory, (a table for finding your data directory can be found under)

| Platform | Value                                    | Example                                           |
| -------- | ---------------------------------------- | ----------------------------------------          |
| Linux    | `$XDG_DATA_HOME` or `$HOME`/.local/share | /home/alice/.local/share/yast/...                 |
| macOS    | `$HOME`/Library/Application Support      | /Users/Alice/Library/Application Support/yast/... |
| Windows  | `{FOLDERID_RoamingAppData}`              | C:\Users\Alice\AppData\Roaming\yast\...           |

(taken from [dirs](https://docs.rs/dirs/latest/dirs/fn.data_dir.html))

You should now have:
- <DATA DIRECTORY PATH>/
  - ...
  - yast/
    - *components/*
    - *lib/*
  - ...

### Platform-specific instructions

#### Linux: Wayland

You *may* need to run YAST as a user in the `input` group for global hotkeys to work.
```sh
sudo usermod -a -G input <user>
```

### Usage

#### YAST

- Right clicking anywhere opens the context menu
- Hotkeys:
  - Ctrl+S: Split or Start
  - Ctrl+R: Reset (with saving attempt)
  - Ctrl+P: Pause

#### YASLE

I'm a teapot

## Changelog

versioning: YY.MMDD-hh

- 26.0209-2(1/2)xx: Initial Prototype
