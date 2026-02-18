local labeled = require("labeled")
local time = require("time")

return {
    ["name"] = "Current Timing Method",
    ["author"] = "yyna",
    ["settings"] =
        function()
            return settings_factory()
                :plugin(labeled.plugin("Current Timing Method"))
                :color("Value Text: Color", 1, 1, 1, 1)
        end,
    ["widget"] =
        function()
            local c = {}
            for i = 1, children.len do
                table.insert(c, children.get(i))
            end

            return labeled.apply(
                widgets.text(snapshot.current_timing_method):style(setting("Value Text: Color"))
            )
        end
}
