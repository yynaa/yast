local sizing = require("sizing")
local background = require("background")
local text = require("text")
local time = require("time")

--- @param segment_index number
local function build_segment_background(segment_index)
	if segment_index > #run.segments then
		return widgets.space():width("shrink"):height("fill"):into()
	end

	if snapshot.current_split ~= nil and segment_index == snapshot.current_split then
		return widgets
			.container(widgets.space():width("shrink"):height("fill"):into())
			:style({ 0, 0, 0, 0 }, setting("Current Segment Background Color"))
			:into()
	else
		return widgets.space():width("shrink"):height("fill"):into()
	end
end

--- @param segment_index number
local function build_segment_icon(segment_index)
	local inner
	if segment_index > #run.segments then
		inner = widgets.space():width("shrink"):height("fill"):into()
	else
		local segment = run.segments[segment_index]
		if segment.icon ~= nil then
			inner = widgets.image(segment.icon):width("shrink"):height("fill"):content_fit("cover"):into()
		else
			inner = widgets.space():width("shrink"):height("fill"):into()
		end
	end
	return widgets.container(inner):padding(5, 0, 5, 0):width("shrink"):height("fill"):into()
end

--- @param segment_index number
local function build_segment_name(segment_index)
	local inner
	if segment_index > #run.segments then
		inner = widgets.space():width("shrink"):height("fill"):into()
	else
		local segment = run.segments[segment_index]
		inner = text.write(segment.name, "Split Name")
			:style(text.color("Split Name"))
			:align_x("left")
			:align_y(text.align_y("Split Name"))
			:width("shrink")
			:height("fill")
			:into()
	end
	return widgets.container(inner):padding(5, 0, 5, 0):width("shrink"):height("fill"):into()
end

--- @param segment_index number
local function build_segment_column(segment_index, column_index)
	local inner
	if segment_index > #run.segments then
		inner = widgets.space():width("shrink"):height("fill"):into()
	else
		local text_content = ""
		local text_color = text.color("Column")
		local text_type = setting("Column " .. math.tointeger(column_index) .. " Type")
		local comparison = snapshot.current_comparison
		if setting("Column " .. math.tointeger(column_index) .. " Comparison") ~= "Current Comparison" then
			comparison = setting("Column " .. math.tointeger(column_index) .. " Comparison")
		end
		local timing_method = setting("Column " .. math.tointeger(column_index) .. " Timing Method")
		if text_type == "Split Time" then
			text_content = time.format(
				time.live_split_time(segment_index, comparison, timing_method),
				setting("Column " .. math.tointeger(column_index) .. " Decimals")
			)
		elseif text_type == "Segment Time" then
			text_content = time.format(
				time.live_segment_time(segment_index, comparison, timing_method),
				setting("Column " .. math.tointeger(column_index) .. " Decimals")
			)
		elseif text_type == "Split Delta" then
			local delta = time.live_delta(segment_index, comparison, timing_method)
			text_content = time.format_delta(delta, setting("Column " .. math.tointeger(column_index) .. " Decimals"))
			if delta ~= nil then
				if delta <= 0 then
					text_color = setting("Delta Color Ahead")
				else
					text_color = setting("Delta Color Behind")
				end
			end
		elseif text_type == "Segment Delta" then
			local delta = time.live_segment_delta(segment_index, comparison, timing_method)
			text_content = time.format_delta(delta, setting("Column " .. math.tointeger(column_index) .. " Decimals"))
			if delta ~= nil then
				if delta <= 0 then
					text_color = setting("Delta Color Ahead")
				else
					text_color = setting("Delta Color Behind")
				end
			end
		end

		inner = text.write(text_content, "Column")
			:style(text_color)
			:align_x("right")
			:align_y(text.align_y("Column"))
			:width("shrink")
			:height("fill")
			:into()
	end

	return widgets.container(inner):padding(5, 0, 5, 0):width("shrink"):height("fill"):into()
end

return {
	name = "Splits",
	author = "yyna",
	settings = function()
		local result = settings_factory()
			:header("Position & Size")
			:plugin(sizing.plugin)
			:plugin(background.plugin)
			:header("Splits")
			:number("Total Splits", 10)
			:number("Upcoming Splits", 1)
			:boolean("Always Show Last Split", true)
			:color("Current Segment Background Color", 0, 0, 1, 1)
			:boolean("Show Separators", true)
			:plugin(text.plugin("Split Name", true, false, true))
			:header("Columns")
			:plugin(text.plugin("Column", true, false, true))
			:color("Delta Color Ahead", 0, 1, 0, 1)
			:color("Delta Color Behind", 1, 0, 0, 1)
			:number_range("Column Count", 0, 8, 1, 1)

		for i = 1, 8 do
			local show_if = function(s)
				return i <= s("Column Count")
			end
			result = result
				:options(
					"Column " .. i .. " Type",
					{ "Split Time", "Segment Time", "Split Delta", "Segment Delta" },
					"Split Time",
					show_if
				)
				:options(
					"Column " .. i .. " Comparison",
					{ "Current Comparison", "Best Segments" },
					"Current Comparison",
					show_if
				)
				:options("Column " .. i .. " Timing Method", { "RealTime", "GameTime" }, "RealTime", show_if)
				:number_range("Column " .. i .. " Decimals", 0, 3, 1, 1, show_if)
		end

		return result
	end,
	widget = function()
		local segment_center = snapshot.current_split
		if segment_center == nil then
			segment_center = 0
		end

		local range = setting("Total Splits")
		if range < #run.segments and setting("Always Show Last Split") then
			range = range - 1
		end

		local range_end = segment_center + setting("Upcoming Splits")
		local range_start = range_end - range + 1

		if range_end > #run.segments then
			range_start = range_start - (range_end - #run.segments)
			range_end = #run.segments
		end
		if range_start < 1 then
			range_end = range_end - range_start + 1
			range_start = 1
		end

		if range < #run.segments and setting("Always Show Last Split") and range_end == #run.segments then
			range = range + 1
			range_start = range_start - 1
		end

		local splits_indexes = {}
		for i = range_start, range_end do
			table.insert(splits_indexes, i)
		end
		if range_end < #run.segments and setting("Always Show Last Split") then
			table.insert(splits_indexes, #run.segments)
		end

		local stack = {}

		local splits_background = {}
		for i, si in ipairs(splits_indexes) do
			table.insert(splits_background, build_segment_background(si))
			if setting("Show Separators") and i < #splits_indexes then
				table.insert(
					splits_background,
					widgets
						.container(widgets.space():width("fill"):height("fill"):into())
						:width("fill")
						:height("fixed", 1)
						:style({ 0, 0, 0, 0 }, { 0.5, 0.5, 0.5, 0.5 })
						:into()
				)
			end
		end
		table.insert(stack, widgets.column(splits_background):width("fill"):height("fill"):into())

		local splits_content = {}

		local splits_icon = {}
		for i, si in ipairs(splits_indexes) do
			table.insert(splits_icon, build_segment_icon(si))
			if setting("Show Separators") and i < #splits_indexes then
				table.insert(
					splits_icon,
					widgets
						.container(widgets.space():width("fill"):height("fill"):into())
						:width("fill")
						:height("fixed", 1)
						:into()
				)
			end
		end
		table.insert(splits_content, widgets.column(splits_icon):width("shrink"):height("fill"):into())

		local splits_name = {}
		for i, si in ipairs(splits_indexes) do
			table.insert(splits_name, build_segment_name(si))
			if setting("Show Separators") and i < #splits_indexes then
				table.insert(
					splits_name,
					widgets
						.container(widgets.space():width("fill"):height("fill"):into())
						:width("fill")
						:height("fixed", 1)
						:into()
				)
			end
		end
		table.insert(splits_content, widgets.column(splits_name):width("fill"):height("fill"):into())

		table.insert(splits_content, widgets.space():width("fill"):height("fill"):into())

		for i = setting("Column Count"), 1, -1 do
			local splits_column = {}
			for _, si in ipairs(splits_indexes) do
				table.insert(splits_column, build_segment_column(si, i))
				if setting("Show Separators") and i < #splits_indexes then
					table.insert(
						splits_column,
						widgets
							.container(widgets.space():width("fill"):height("fill"):into())
							:width("fill")
							:height("fixed", 1)
							:into()
					)
				end
			end
			table.insert(
				splits_content,
				widgets.column(splits_column):align_x("right"):width("shrink"):height("fill"):into()
			)
		end

		table.insert(
			stack,
			widgets.row(splits_content):width("fill"):height("fill"):spacing(5):padding(0, 5, 0, 5):into()
		)

		return widgets.stack(stack):width("fill"):height("fill"):into()
	end,
}
