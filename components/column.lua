-- a test component

return {
  ["name"] = "Column",
  ["author"] = "Built-in",
  ["settings"] = build_settings(),
  ["widget"] =
    function(children_count)
      return widgets
        .text("children count: " .. tostring(children_count))
        :width("fill")
        :height("fill")
        :align_x("center")
        :align_y("center")
        :into()
    end
}
