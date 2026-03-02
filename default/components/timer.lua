local sizing = require("sizing")
local background = require("background")
local text = require("text")
local time = require("time")

return {
	name = "Timer",
	author = "yyna",
	settings = function()
		return settings_factory()
			:header("Position & Size")
			:plugin(sizing.plugin)
			:plugin(background.plugin)
			:header("Timer")
			:plugin(text.plugin("Timer", false, true, true))
			:number_range("Decimals", 0, 3, 1, 2)
			:header("Colors")
			:color("Color NotRunning", 0.5, 0.5, 0.5, 1)
			:color("Color Ended", 0.5, 0.5, 1, 1)
			:color("Color Paused", 0.3, 0.3, 0.5, 1)
			:color("Color Running Ahead", 0, 1, 0, 1)
			:color("Color Running Behind", 1, 0, 0, 1)
	end,
	widget = function()
		local time_number = time.current_time()
		local time_string = time.format(time_number, setting("Decimals"))

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

		return sizing.apply(
			background.apply(
				text.write(time_string, "Timer")
					:width("fill")
					:height("fill")
					:align_x(text.align_x("Timer"))
					:align_y(text.align_y("Timer"))
					:style(setting(color))
					:into()
			)
		)
	end,
}
