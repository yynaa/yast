local sizing = require("sizing")
local background = require("background")
local time = require("time")

return {
  ["name"] = "Timer",
  ["author"] = "yyna",
  ["settings"] = function()
    return settings_factory()
      :header("Position & Size")
      :plugin(sizing.plugin)
      :plugin(background.plugin)
      :header("Timer")
      :string("Font", "Arial")
      :number("Text Size", 60)
      :options("Text Align X", {"Left", "Center", "Right"}, "Center")
      :options("Text Align Y", {"Top", "Center", "Bottom"}, "Center")
      :number_range("Decimals", 0, 3, 1, 2)
      :header("Colors")
      :color("Color NotRunning", 0.5, 0.5, 0.5, 1)
      :color("Color Ended", 0.5, 0.5, 1, 1)
      :color("Color Paused", 0.3, 0.3, 0.5, 1)
      :color("Color Running Ahead", 0, 1, 0, 1)
      :color("Color Running Behind", 1, 0, 0, 1)
    end,
  ["widget"] =
    function()
      local time_number = time.cta(snapshot.current_time)
      local time_string = require("time").format(time_number, setting("Decimals"))
      
      local color = "Color " .. snapshot.current_phase
      local ld = time.live_delta()
      if snapshot.current_phase == "Running" then
        if ld and time.cta(ld) ~= nil then
          if time.cta(ld) < 0 then
            color = color .. " Ahead"
          else
            color = color .. " Behind"
          end
        else
          color = "Color NotRunning"
        end
      end
      
      return sizing.apply(background.apply(
        widgets
        .text(time_string)
        :width("fill")
        :height("fill")
        :align_x(string.lower(setting("Text Align X")))
        :align_y(string.lower(setting("Text Align Y")))
        :font(setting("Font"))
        :size(setting("Text Size"))
        :style(setting(color))
        :into()
      ))
    end
}
