# YAST, and the YASX ecosystem

**YAST** (Yet Another Speedrunning Timer || "fast" and "yyna" combined) is a native multi-platform frontend for
livesplit-core written in Rust.  
**YASLE** (Yet Another Speedrunning Layout Editor) is YAST's layout editor.

***YAST** and **YASLE** form the **YASX ecosystem**, your multi-platform speedrunning timer!*

> [!WARNING]
> **The YASX ecosystem is in alpha!**  
> This means **major changes** could occur to the Layout Lua API, which means:
> - as a user, components may break when upgrading the YASX-ecosystem.
> - as a developer, you may have a fix your components on new updates.

## Rationale

- Cross-platform timer based on LiveSplit through livesplit-core
- Interoperability for splits with LiveSplit and LiveSplit One
- Increase of options for Layout placements, through the power of [Iced](https://iced.rs) widgets
- Ease of development for Layout through Lua scripting

## Features

✅ = ok, ⌛ = untested, ⚠️ = partial, 🔴 = todo

- ✅ Cross-platform
    - ✅ Linux
        - ✅ X11
        - ✅ Wayland
    - ✅ Windows
    - ✅ MacOS
- ⌛ Interoperable with LiveSplit One
    - ✅ Base splits
    - ⌛ Autosplitting
- ✅ Global Hotkeys
- ✅ Scripting for Layout components
- ✅ Layout Editor
- 🔴 Layout Converters
    - 🔴 LiveSplit
    - 🔴 LiveSplit One
- 🔴 Splits Editor (use [LSO Web](https://one.livesplit.org) until then)

## Installing

> [!IMPORTANT]
> I'm looking for package maintainers to put YAST on their favorite PMs!  
> If you wish to inform me of such, please contact me through [email](mailto://me@yyna.xyz)
> or [other means](https://yyna.xyz/c)!  
> Thank you so much!

Head over to the [releases](https://github.com/yynaa/yast/releases), find your operating system, and off you go!

Don't know what your operating system is? Here is a list to help you figure it out:

- Windows: `x86_64-pc-windows-gnu`
- Linux: `x86_64-unknown-linux-gnu`
- Mac: `aarch64-apple-darwin`

Is your operating system not listed here? Are you struggling to make the YASX ecosystem run on your machine?  
You may look into building it yourself.

### Platform-specific instructions

#### Linux: Wayland

You need to run YAST as a user in the `input` group for global hotkeys to work.  
Running YAST as root is not recommended.

#### MacOS

You need to allow keyboard accessibility to YAST, through **System Preferences > Security & Privacy > Privacy >
Accessibility**.  
If you are running YAST in a terminal, allow your terminal instead.

## Usage

todo!()

## Building from source

```sh
git clone https://github.com/yynaa/yast
cd yast
cargo build --release -p yast
cargo build --release -p yasle
# you can now find yast and yasle in target/release/yas[x]
```