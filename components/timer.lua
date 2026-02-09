local spacing = require("spacing")

return {
  ["name"] = "Timer",
  ["author"] = "yyna",
  ["settings"] = build_settings()
    :plugin(spacing.plugin)
    :string("Font", "Arial")
    :number("Text Size", 60)
    :options("Text Align X", "Center", {"Left", "Center", "Right"})
    :options("Text Align Y", "Center", {"Top", "Center", "Bottom"}),
  ["widget"] =
    function()
      return spacing.apply(
        widgets
        .text(tostring(snapshot.current_time.real_time))
        :width("fill")
        :height("fill")
        :align_x(string.lower(settings:get("Text Align X")))
        :align_y(string.lower(settings:get("Text Align Y")))
        :font(settings:get("Font"))
        :size(settings:get("Text Size"))
        :into()
      )
    end
}
