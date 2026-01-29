-- a test component

return {
  ["name"] = "Test Component",
  ["author"] = "yyna",
  ["settings"] = build_settings():boolean("test bool", false),
  ["widget"] =
    function()
      return widgets
        .text(tostring(snapshot.current_time.real_time))
        :color(1.0, 0.0, 0.0, 1.0):width("fill")
        :height("fill")
        :align_x("center")
        :align_y("center")
        :into()
    end
}
