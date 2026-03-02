local labeled = require("labeled")
local time = require("time")

return {
	name = "Sum of Best",
	author = "yyna",
	settings = function()
		return settings_factory()
			:plugin(labeled.plugin("Sum of Best"))
			:color("Value Text: Color", 1, 1, 1, 1)
			:number_range("Value Text: Decimals", 0, 3, 1, 1)
	end,
	widget = function()
		return labeled.apply(
			widgets
				.text(time.format(time.sum_of_best(), setting("Value Text: Decimals")))
				:style(setting("Value Text: Color"))
		)
	end,
}
