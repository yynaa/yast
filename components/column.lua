return {
  ["name"] = "Column",
  ["author"] = "Built-in",
  ["settings"] = build_settings(),
  ["widget"] =
    function(children_count)
      local c = {}
      for i = 1,children_count do
        table.insert(c, children.get(i))
      end

      return widgets
        .column(c)
        :into()
    end
}
