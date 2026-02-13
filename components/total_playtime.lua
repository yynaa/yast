local labeled = require("labeled")
local time = require("time")

return {
  ["name"] = "Total Playtime",
  ["author"] = "yyna",
  ["settings"] =
  function()
    return settings_factory()
      :plugin(labeled.plugin("Total Playtime"))
      :color("Value Text: Color",1,1,1,1)
  end,
  ["widget"] =
    function()
      local c = {}
      for i = 1,children.len do
        table.insert(c, children.get(i))
      end

      return labeled.apply(
        widgets.text(time.format(analysis.total_playtime, 0)):style(setting("Value Text: Color"))
      )
    end
}
