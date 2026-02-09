# YAST

YAST (Yet Another Speedrunning Timer || "fast" and "yyna" combined) is a native multiplatform frontend for livesplit-core written in Rust.

> [!CAUTION]
> WORK IN PROGRESS

## Installing

There are currently no releases, you need to build YAST yourself.

## Building from source

```sh
cargo build
# outputs `yast` and `yast-layout-editor`
```

### Installing default components

YAST comes with no default layout components, you need to install them yourself.  
You need to copy the contents of `components/` and `lib/` in your data directory, under the `yast` folder.

| Platform | Value                                    | Example                                           |
| -------- | ---------------------------------------- | ----------------------------------------          |
| Linux    | `$XDG_DATA_HOME` or `$HOME`/.local/share | /home/alice/.local/share/yast/...                 |
| macOS    | `$HOME`/Library/Application Support      | /Users/Alice/Library/Application Support/yast/... |
| Windows  | `{FOLDERID_LocalAppData}`                | C:\Users\Alice\AppData\Local\yast\...             |

(taken from [dirs](https://docs.rs/dirs/latest/dirs/fn.data_dir.html))

## Usage

### Platform-specific instructions

#### Wayland

You *may* need to run YAST as a user in the `input` group for global hotkeys to work.
