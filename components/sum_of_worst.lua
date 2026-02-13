local labeled = require("labeled")
local time = require("time")

return {
  ["name"] = "Sum of Worst",
  ["author"] = "yyna",
  ["settings"] =
  function()
    return settings_factory()
      :plugin(labeled.plugin("Sum of Worst"))
      :color("Value Text: Color",1,1,1,1)
      :number_range("Value Text: Decimals", 0, 3, 1, 1)
  end,
  ["widget"] =
    function()
      local c = {}
      for i = 1,children.len do
        table.insert(c, children.get(i))
      end

      return labeled.apply(
        widgets.text(time.format(time.cta(analysis.sum_of_worst_segments), setting("Value Text: Decimals"))):style(setting("Value Text: Color"))
      )
    end
}
