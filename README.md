# YAST

YAST (Yet Another Speedrunning Timer || "fast" and "yyna" combined) is a native multiplatform frontend for livesplit-core written in Rust.

> [!CAUTION]
> WORK IN PROGRESS

## Rationale

- Cross-platform timer based on the most widely used timer in the world
- Interoperability with said timer
- Ease of development for Layout through Lua scripting

## Features

✅ = ok, ⌛ = untested, ⚠️ = partial, 🔴 = todo

- ⌛ Cross-platform
  - ✅ Linux
    - ✅ X11
    - ✅ Wayland
  - ⌛ Windows
  - ✅ MacOS
- ⚠️ Interoperable with LiveSplit One
  - ✅ Base splits
  - 🔴 Autosplitting
- ✅ Scripting for Layout components
- ⚠️ Default Layout components
  - ⚠️ Libraries
  - ⚠️ Containers
  - ⚠️ Information
- ✅ Layout Editor
- 🔴 Splits Editor (use [LSO](https://one.livesplit.org) until then)

## Installing

There are currently no releases, you need to build YAST yourself.

## Building from source

```sh
cargo build
# outputs `yast` and `yast-layout-editor`
```

### Installing default components

YAST comes with no default layout components, you need to install them yourself.  
You need to copy `components/` and `lib/` in your data directory, under the `yast` folder.

| Platform | Value                                    | Example                                           |
| -------- | ---------------------------------------- | ----------------------------------------          |
| Linux    | `$XDG_DATA_HOME` or `$HOME`/.local/share | /home/alice/.local/share/yast/...                 |
| macOS    | `$HOME`/Library/Application Support      | /Users/Alice/Library/Application Support/yast/... |
| Windows  | `{FOLDERID_RoamingAppData}`              | C:\Users\Alice\AppData\Roaming\yast\...           |

(taken from [dirs](https://docs.rs/dirs/latest/dirs/fn.data_dir.html))

## Usage

### Platform-specific instructions

#### Wayland

You *may* need to run YAST as a user in the `input` group for global hotkeys to work.
