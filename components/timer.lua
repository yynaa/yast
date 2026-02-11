local sizing = require("sizing")
local background = require("background")

return {
  ["name"] = "Timer",
  ["author"] = "yyna",
  ["settings"] = function()
    return settings_factory()
      :plugin(sizing.plugin)
      :plugin(background.plugin)
      :header("Timer")
      :string("Font", "Arial")
      :number("Text Size", 60)
      :options("Text Align X", {"Left", "Center", "Right"}, "Center")
      :options("Text Align Y", {"Top", "Center", "Bottom"}, "Center")
      :color("Color NotRunning", 0.5, 0.5, 0.5, 1)
      :color("Color Running", 0.5, 1, 0.5, 1)
      :color("Color Ended", 0.5, 0.5, 1, 1)
      :color("Color Paused", 0.3, 0.5, 0.3, 1)
      :number_range("Decimals", 2, 0, 3, 1)
    end,
  ["widget"] =
    function()
      local time = snapshot.current_time.real_time
      if snapshot.current_timing_method == "GameTime" then time = snapshot.current_time.game_time end
      
      local time_string = require("time").format(time, setting("Decimals"))
      
      return sizing.apply(background.apply(
        widgets
        .text(time_string)
        :width("fill")
        :height("fill")
        :align_x(string.lower(setting("Text Align X")))
        :align_y(string.lower(setting("Text Align Y")))
        :font(setting("Font"))
        :size(setting("Text Size"))
        :style(setting("Color " .. snapshot.current_phase))
        :into()
      ))
    end
}
