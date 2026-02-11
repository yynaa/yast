return {
  ["name"] = "Stack",
  ["author"] = "yyna",
  ["settings"] =
    function()
      return settings_factory()
    end,
  ["widget"] =
    function()
      local c = {}
      for i = 1,children.len do
        table.insert(c, children.get(i))
      end

      return widgets
        .stack(c)
        :width("fill")
        :height("fill")
        :into()
    end
}
