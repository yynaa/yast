local sizing = require("sizing")
local background = require("background")
local text = require("text")
local time = require("time")

return {
	name = "Detailed Timer",
	author = "yyna",
	settings = function()
		local settings = settings_factory()
			:header("Position & Size")
			:plugin(sizing.plugin)
			:plugin(background.plugin)
			:header("Information")
			:boolean("Show Segment Timer", true)
			:boolean("Show Split Name", true)
			:boolean("Show Split Icon", true)
			:number_range("Comparison Count", 0, 3, 1, 2)
			:header("Timer")
			:plugin(text.plugin("Timer", false, false, false))
			:number_range("Timer Decimals", 0, 3, 1, 2)
			:plugin(text.plugin("Segment Timer", true, false, false, function(s)
				return s("Show Segment Timer")
			end))
			:number_range("Segment Timer Decimals", 0, 3, 1, 1, function(s)
				return s("Show Segment Timer")
			end)
			:plugin(text.plugin("Split Name", true, false, false, function(s)
				return s("Show Split Name")
			end))
			:plugin(text.plugin("Comparisons", true, false, true))

		for i = 1, 3 do
			local show_if = function(s)
				return i <= s("Comparison Count")
			end
			settings = settings
				:string("Comparison " .. i, "Best Segments", show_if)
				:options(
					"Comparison " .. i .. " Timing Method",
					{ "Current", "RealTime", "GameTime" },
					"Current",
					show_if
				)
				:number_range("Comparison " .. i .. " Decimals", 0, 3, 1, 1, show_if)
		end

		settings = settings
			:header("Colors")
			:color("Color NotRunning", 0.5, 0.5, 0.5, 1)
			:color("Color Ended", 0.5, 0.5, 1, 1)
			:color("Color Paused", 0.3, 0.3, 0.5, 1)
			:color("Color Running Ahead", 0, 1, 0, 1)
			:color("Color Running Behind", 1, 0, 0, 1)

		return settings
	end,
	widget = function()
		local time_number = time.current_time()
		local time_string = time.format(time_number, setting("Timer Decimals"))

		local color = "Color " .. snapshot.current_phase
		local ld = time.live_delta()
		if snapshot.current_phase == "Running" then
			if ld ~= nil then
				if ld < 0 then
					color = color .. " Ahead"
				else
					color = color .. " Behind"
				end
			else
				color = "Color NotRunning"
			end
		end

		local timer = text.write(time_string, "Timer")
			:width("fill")
			:height("fill")
			:align_x("right")
			:align_y("center")
			:style(setting(color))
			:into()
		local timer_vec = { timer }

		local segment_timer
		if setting("Show Segment Timer") then
			local segment_time_string = time.format(time.current_segment_time(), setting("Segment Timer Decimals"))
			segment_timer = text.write(segment_time_string, "Segment Timer")
				:width("fill")
				:height("fill")
				:align_x("right")
				:align_y("center")
				:style(text.color("Segment Timer"))
				:into()
			table.insert(timer_vec, segment_timer)
		end

		local timer_data_column = widgets
			.container(widgets.column(timer_vec):width("fill"):height("shrink"):align_x("right"):spacing(2):into())
			:width("fill")
			:height("fill")
			:align_x("right")
			:align_y("center")
			:padding(5, 5, 5, 5)
			:into()

		local global_stack_vec = {}

		local cs = snapshot.current_split
		if cs and cs <= #run.segments then
			local segment_info_row_vec = {}

			if setting("Show Split Icon") and run.segments[cs].icon ~= nil then
				table.insert(
					segment_info_row_vec,
					widgets.image(run.segments[cs].icon):width("shrink"):height("fill"):content_fit("contain"):into()
				)
			end

			local segment_info_col_vec = {}
			if setting("Show Split Name") then
				table.insert(
					segment_info_col_vec,
					text.write(run.segments[cs].name, "Split Name")
						:style(text.color("Split Name"))
						:align_x("left")
						:align_y("top")
						:into()
				)
			end
			table.insert(segment_info_col_vec, widgets.space():height("fill"):into())
			for i = 1, setting("Comparison Count") do
				local comparison = setting("Comparison " .. math.tointeger(i))
				if comparison == "Current Comparison" then
					comparison = snapshot.current_comparison
				end
				if run.segments[cs].comparisons[comparison] ~= nil then
					local timing_method = snapshot.current_timing_method
					if setting("Comparison " .. math.tointeger(i) .. " Timing Method") ~= "Current" then
						timing_method = setting("Comparison " .. math.tointeger(i) .. " Timing Method")
					end
					local decimals = setting("Comparison " .. math.tointeger(i) .. " Decimals")
					local text_content = comparison
						.. ": "
						.. time.format(time.live_segment_time(nil, comparison, timing_method), decimals)
					table.insert(
						segment_info_col_vec,
						text.write(text_content, "Comparisons")
							:style(text.color("Comparisons"))
							:align_x("left")
							:align_y("bottom")
							:into()
					)
				else
					log.error(comparison .. " is not a valid comparison")
				end
			end

			table.insert(
				segment_info_row_vec,
				widgets
					.container(widgets.column(segment_info_col_vec):height("fill"):width("shrink"):into())
					:width("fill")
					:height("fill")
					:align_x("left")
					:align_y("center")
					:into()
			)

			local segment_info_row =
				widgets.row(segment_info_row_vec):width("fill"):height("fill"):padding(5, 5, 5, 5):spacing(5):into()

			table.insert(global_stack_vec, segment_info_row)
		end

		table.insert(global_stack_vec, timer_data_column)
		local global_stack = widgets.stack(global_stack_vec):height("fill"):width("fill"):into()

		return sizing.apply(background.apply(global_stack))
	end,
}
