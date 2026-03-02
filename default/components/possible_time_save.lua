local labeled = require("labeled")
local time = require("time")

return {
	name = "Possible Time Save",
	author = "yyna",
	settings = function()
		return settings_factory()
			:plugin(labeled.plugin("Possible Time Save"))
			:color("Value Text: Color", 1, 1, 1, 1)
			:number_range("Value Text: Decimals", 0, 3, 1, 1)
			:header("Information")
			:boolean("Show Total Possible Time Save", false)
	end,
	widget = function()
		local time_number
		if setting("Show Total Possible Time Save") then
			time_number = time.total_possible_time_save()
		else
			time_number = time.possible_time_save(snapshot.current_split)
		end

		return labeled.apply(
			widgets.text(time.format(time_number, setting("Value Text: Decimals"))):style(setting("Value Text: Color"))
		)
	end,
}
