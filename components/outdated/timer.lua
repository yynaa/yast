local sizing = require("sizing")
local background = require("background")

return {
  ["name"] = "Timer",
  ["author"] = "yyna",
  ["settings"] = build_settings()
    :plugin(sizing.plugin)
    :plugin(background.plugin)
    :string("Font", "Arial")
    :number("Text Size", 60)
    :options("Text Align X", "Center", {"Left", "Center", "Right"})
    :options("Text Align Y", "Center", {"Top", "Center", "Bottom"})
    :color("Color NotRunning", 0.5, 0.5, 0.5, 1)
    :color("Color Running", 0.5, 1, 0.5, 1)
    :color("Color Ended", 0.5, 0.5, 1, 1)
    :color("Color Paused", 0.3, 0.5, 0.3, 1)
    :number_range("Decimals", 2, 0, 3, 1),
  ["widget"] =
    function()
      local time = snapshot.current_time.real_time
      if snapshot.current_timing_method == "GameTime" then time = snapshot.current_time.game_time end
      
      local time_string = require("time").format(time, settings:get("Decimals"))
      
      return sizing.apply(background.apply(
        widgets
        .text(time_string)
        :width("fill")
        :height("fill")
        :align_x(string.lower(settings:get("Text Align X")))
        :align_y(string.lower(settings:get("Text Align Y")))
        :font(settings:get("Font"))
        :size(settings:get("Text Size"))
        :style(settings:get("Color " .. snapshot.current_phase))
        :into()
      ))
    end
}
