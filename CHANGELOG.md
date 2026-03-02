# YASX ecosystem changelog

- Changelogs
    - [YAST - `yast`](#yast---yast)
    - [YASLE - `yasle`](#yasle---yasle)
    - [`yast-core`](#yast-core)
    - [`yast-windows`](#yast-windows)
- [Version Map](#version-map)

## YAST - `yast`

### **0.1.1** - 2026-02-27

#### Added

- Confirmation before closing for saving splits/layout

### **0.1.0** - 2026-02-16

#### Added

- Splits loading/saving
- Layout loading/saving
- Autosplitter loading
- Global Hotkeys

## YASLE - `yasle`

### **0.1.1** - 2026-02-27

#### Added

- Confirmation before closing for saving layout

### **0.1.0** - 2026-02-16

#### Added

- Basic layout editor
- Basic hotkey editor
- Layout previewer

## `yast-core`

### Unreleased

#### Added

- `time` library modified
  - See the library's documentation for more information
- Logging functions for Lua

#### Changed

- All default components rewritten

#### Depreciated

- In the `time` library, `cta`, `current_timing_accessor` and `accessor_*` operations are depreciated.
  - Please check the library, as most functions regarding accessing timer values are now handled by the library itself.

### **0.1.0** - 2026-02-16

#### Added

- Layout bricks
- Lua context
- Default components initialization
- App repository

## `yast-windows`

### **0.1.0** - 2026-02-16

#### Added

- `iced` to `handy-keys` hotkey converter

## Version Map

| `yast` | `yasle` | `yast-core` | `yast-windows` |
|:-------|:--------|:------------|:---------------|
| 0.1.1  | 0.1.1   | 0.1.0       | 0.1.0          |
| 0.1.0  | 0.1.0   | 0.1.0       | 0.1.0          |
