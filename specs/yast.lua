--- @class settings
--- @field get fun(self: settings, param_name: string): any
--- @field plugin fun(self: settings, plugin: fun(s: settings): settings): settings
--- @field boolean fun(self: settings, param_name: string, default: boolean): settings
--- @field string fun(self: settings, param_name: string, default: string): settings
--- @field options fun(self: settings, param_name: string, default: string, options: string[]): settings
--- @field number fun(self: settings, param_name: string, default: number): settings
--- @field number_range fun(self: settings, param_name: string, default: number, min: number, max: number, step: number): settings
--- @field color fun(self: settings, param_name: string, default_r: number, default_g: number, default_b: number, default_a: number): settings
--- @field image fun(self: settings, param_name: string): settings
settings = {}

--- @return settings
function build_settings() end

--- @class snapshot_current_time
--- @field real_time number
--- @field game_time number
local snapshot_current_time = {}

--- @class snapshot
--- @field current_attempt_duration number
--- @field current_comparison string
--- @field current_phase string
--- @field current_split number | nil
--- @field current_timing_method string
--- @field current_time snapshot_current_time
snapshot = {}

--- @class run_metadata
--- @field run_id string
--- @field platform_name string
--- @field uses_emulator boolean
--- @field region_name string
local run_metadata = {}

--- @class run_segment_comparison
--- @field real_time number
--- @field game_time number
local run_segment_comparison = {}

--- @class run_segment
--- @field name string
--- @field icon number[]
--- @field comparisons table<string, run_segment_comparison>
local run_segment = {}

--- @class run
--- @field game_name string
--- @field game_icon number[]
--- @field category_name string
--- @field attempt_count number
--- @field metadata run_metadata
--- @field segments run_segment[]
run = {}

--- @class widgets
widgets = {}

--- @class widget
local widget = {}

--- @class widget_text
--- @field into fun(self: widget_text): widget
--- @field align_x fun(self: widget_text, alignment: "left"|"right"|"center"): widget_text
--- @field align_y fun(self: widget_text, alignment: "top"|"bottom"|"center"): widget_text
--- @field style fun(self: widget_text, color: number[]|nil): widget_text
--- @field size fun(self: widget_text, size: number): widget_text
--- @field width fun(self: widget_text, type: "fill"|"fill_portion"|"shrink"|"fixed", unit: number|nil): widget_text
--- @field height fun(self: widget_text, type: "fill"|"fill_portion"|"shrink"|"fixed", unit: number|nil): widget_text
--- @field font fun(self: widget_text, font: string): widget_text

--- @param content string
--- @return widget_text
function widgets.text(content) end

--- @class widget_image
--- @field into fun(self: widget_image): widget
--- @field width fun(self: widget_image, type: "fill"|"fill_portion"|"shrink"|"fixed", unit: number|nil): widget_image
--- @field height fun(self: widget_image, type: "fill"|"fill_portion"|"shrink"|"fixed", unit: number|nil): widget_image
--- @field content_fit fun(self: widget_image, fit: "contain"|"cover"|"fill"|"none"|"scale_down"): widget_image
--- @field filter_method fun(self: widget_image, method: "linear"|"nearest"): widget_image
--- @field opacity fun(self: widget_image, opacity: number): widget_image
--- @field crop fun(self: widget_image, x: number, y: number, width: number, height: number): widget_image

--- @param handle userdata
--- @return widget_image
function widgets.image(handle) end

--- @class widget_column
--- @field into fun(self: widget_column): widget
--- @field spacing fun(self: widget_column, spacing: number): widget_column
--- @field padding fun(self: widget_column, top: number, right: number, bottom: number, left: number): widget_column
--- @field width fun(self: widget_column, type: "fill"|"fill_portion"|"shrink"|"fixed", unit: number|nil): widget_column
--- @field height fun(self: widget_column, type: "fill"|"fill_portion"|"shrink"|"fixed", unit: number|nil): widget_column
--- @field align_x fun(self: widget_column, alignment: "left"|"right"|"center"): widget_column
--- @field clip fun(self: widget_column, clip: boolean): widget_column

--- @param children widget[]
--- @return widget_column
function widgets.column(children) end

--- @class widget_row
--- @field into fun(self: widget_row): widget
--- @field spacing fun(self: widget_row, spacing: number): widget_row
--- @field padding fun(self: widget_row, top: number, right: number, bottom: number, left: number): widget_row
--- @field width fun(self: widget_row, type: "fill"|"fill_portion"|"shrink"|"fixed", unit: number|nil): widget_row
--- @field height fun(self: widget_row, type: "fill"|"fill_portion"|"shrink"|"fixed", unit: number|nil): widget_row
--- @field align_y fun(self: widget_row, alignment: "top"|"bottom"|"center"): widget_row
--- @field clip fun(self: widget_row, clip: boolean): widget_row

--- @param children widget[]
--- @return widget_row
function widgets.row(children) end

--- @class widget_container
--- @field into fun(self: widget_container): widget
--- @field padding fun(self: widget_container, top: number, right: number, bottom: number, left: number): widget_container
--- @field width fun(self: widget_container, type: "fill"|"fill_portion"|"shrink"|"fixed", unit: number|nil): widget_container
--- @field height fun(self: widget_container, type: "fill"|"fill_portion"|"shrink"|"fixed", unit: number|nil): widget_container
--- @field align_x fun(self: widget_container, alignment: "left"|"right"|"center"): widget_container
--- @field align_y fun(self: widget_container, alignment: "top"|"bottom"|"center"): widget_container
--- @field clip fun(self: widget_container, clip: boolean): widget_container
--- @field style fun(self: widget_container, text_color: number[]|nil, background_color: number[]|nil): widget_container

--- @param child widget
--- @return widget_container
function widgets.container(child) end

--- @class widget_stack
--- @field into fun(self: widget_stack): widget
--- @field width fun(self: widget_stack, type: "fill"|"fill_portion"|"shrink"|"fixed", unit: number|nil): widget_stack
--- @field height fun(self: widget_stack, type: "fill"|"fill_portion"|"shrink"|"fixed", unit: number|nil): widget_stack
--- @field clip fun(self: widget_stack, clip: boolean): widget_stack

--- @param children widget[]
--- @return widget_stack
function widgets.stack(children) end

--- @class widget_space
--- @field into fun(self: widget_space): widget
--- @field width fun(self: widget_space, type: "fill"|"fill_portion"|"shrink"|"fixed", unit: number|nil): widget_space
--- @field height fun(self: widget_space, type: "fill"|"fill_portion"|"shrink"|"fixed", unit: number|nil): widget_space

--- @return widget_space
function widgets.space() end

--- @class children
--- @field len number
--- @field get fun(index: number): widget
children = {}
