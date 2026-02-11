local sizing = require("sizing")
local background = require("background")

return {
  ["name"] = "Title",
  ["author"] = "yyna",
  ["settings"] = function()
    return settings_factory()
      :plugin(sizing.plugin)
      :plugin(background.plugin)
      :header("Title")
      :string("Title Font", "Arial")
      :number("Title Size", 30)
      :options("Title Align X", {"Left", "Center", "Right"}, "Center")
      :color("Title Color", 1, 1, 1, 1)
      :string("Category Font", "Arial")
      :number("Category Size", 20)
      :options("Category Align X", {"Left", "Center", "Right"}, "Center")
      :color("Category Color", 1, 1, 1, 1)
    end,
  ["widget"] =
    function()
      return sizing.apply(background.apply(
        widgets.column({
          widgets
          .text(run.game_name)
          :width("fill")
          :height("fill")
          :align_x(string.lower(setting("Title Align X")))
          :font(setting("Title Font"))
          :size(setting("Title Size"))
          :style(setting("Title Color"))
          :into(),
          widgets
          .text(run.category_name)
          :width("fill")
          :height("fill")
          :align_x(string.lower(setting("Category Align X")))
          :font(setting("Category Font"))
          :size(setting("Category Size"))
          :style(setting("Category Color"))
          :into()
        }):width("fill"):height("fill"):into()
      ))
    end
}
