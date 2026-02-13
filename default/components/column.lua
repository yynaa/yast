local background = require("background")

return {
  ["name"] = "Column",
  ["author"] = "yyna",
  ["settings"] =
  function()
    return settings_factory()
      :plugin(background.plugin)
  end,
  ["widget"] =
    function()
      local c = {}
      for i = 1,children.len do
        table.insert(c, children.get(i))
      end

      return background.apply(
        widgets
        .column(c)
        :width("fill")
        :height("fill")
        :into()
      )
    end
}
