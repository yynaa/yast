local sizing = require("sizing")
local background = require("background")

return {
  ["name"] = "Title",
  ["author"] = "yyna",
  ["settings"] = build_settings()
    :plugin(sizing.plugin)
    :plugin(background.plugin)
    :string("Title Font", "Arial")
    :number("Title Size", 30)
    :options("Title Align X", "Center", {"Left", "Center", "Right"})
    :color("Title Color", 1, 1, 1, 1)
    :string("Category Font", "Arial")
    :number("Category Size", 20)
    :options("Category Align X", "Center", {"Left", "Center", "Right"})
    :color("Category Color", 1, 1, 1, 1),
  ["widget"] =
    function()
      return sizing.apply(background.apply(
        widgets.column({
          widgets
          .text(run.game_name)
          :width("fill")
          :height("fill")
          :align_x(string.lower(settings:get("Title Align X")))
          :font(settings:get("Title Font"))
          :size(settings:get("Title Size"))
          :style(settings:get("Title Color"))
          :into(),
          widgets
          .text(run.category_name)
          :width("fill")
          :height("fill")
          :align_x(string.lower(settings:get("Category Align X")))
          :font(settings:get("Category Font"))
          :size(settings:get("Category Size"))
          :style(settings:get("Category Color"))
          :into()
        }):width("fill"):height("fill"):into()
      ))
    end
}
