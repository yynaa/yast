local sizing = require("sizing")

return {
  ["name"] = "Timer",
  ["author"] = "Built-in",
  ["settings"] = build_settings():plugin(sizing.plugin),
  ["widget"] =
    function()
      return sizing.sizer(widgets
        .text(tostring(snapshot.current_time.real_time))
        :width("fill")
        :height("fill")
        :align_x("center")
        :align_y("center")
        :into())
    end
}
