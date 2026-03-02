local labeled = require("labeled")
local time = require("time")

return {
	name = "Current Comparison",
	author = "yyna",
	settings = function()
		return settings_factory():plugin(labeled.plugin("Current Comparison")):color("Value Text: Color", 1, 1, 1, 1)
	end,
	widget = function()
		return labeled.apply(widgets.text(snapshot.current_comparison):style(setting("Value Text: Color")))
	end,
}
