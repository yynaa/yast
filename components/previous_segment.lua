local labeled = require("labeled")
local time = require("time")

return {
  ["name"] = "Previous Segment",
  ["author"] = "yyna",
  ["settings"] =
  function()
    return settings_factory()
      :plugin(labeled.plugin("Previous Segment"))
      :color("Value Text: Color Not Running", 1, 1, 1, 1)
      :color("Value Text: Color Ahead", 0, 1, 0, 1)
      :color("Value Text: Color Behind", 1, 0, 0, 1)
      :number_range("Value Text: Decimals", 0, 3, 1, 1)
  end,
  ["widget"] =
    function()
      local c = {}
      for i = 1,children.len do
        table.insert(c, children.get(i))
      end
      
      local delta = nil
      if snapshot.current_split ~= nil and snapshot.current_split > 1 then
        delta = analysis.comparisons[snapshot.current_comparison].segments[snapshot.current_split-1].last_delta
      end
      
      local delta_number = nil
      if delta ~= nil then
        delta_number = time.cta(delta)
      end
      
      local color = "Not Running"
      if delta_number ~= nil then
        if delta_number >= 0 then
          color = "Behind"
        else
          color = "Ahead"
        end
      end
      color = "Value Text: Color " .. color

      return labeled.apply(
        widgets.text(time.format_delta(delta_number, setting("Value Text: Decimals"))):style(setting(color))
      )
    end
}
