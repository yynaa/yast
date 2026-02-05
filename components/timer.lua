local sizing = require("sizing")

return {
  ["name"] = "Timer",
  ["author"] = "yyna",
  ["settings"] = build_settings():plugin(sizing.plugin),
  ["widget"] =
    function()
      return widgets
        .text(tostring(snapshot.current_time.real_time))
        :width(sizing.get_width())
        :height(sizing.get_height())
        :align_x("center")
        :align_y("center")
        :into()
    end
}
