return {
  ["name"] = "Row",
  ["author"] = "yyna",
  ["settings"] = build_settings(),
  ["widget"] =
    function()
      local c = {}
      for i = 1,children.len do
        table.insert(c, children.get(i))
      end

      return widgets
        .row(c)
        :width("fill")
        :height("fill")
        :into()
    end
}
