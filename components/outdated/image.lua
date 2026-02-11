local spacing = require("spacing")

return {
  ["name"] = "Image",
  ["author"] = "yyna",
  ["settings"] = build_settings():plugin(spacing.plugin):image("Image"):number_range("Opacity", 100., 0., 100., 1.),
  ["widget"] =
    function()
      local stack_vec = {
        widgets
          .container(widgets.space():width("fill"):height("fill"):into())
          :width("fill")
          :height("fill")
          :style(nil, {0, 0, 0, 255})
          :into()
      }

      local image = settings:get("Image")
      if image ~= nil then
        table.insert(stack_vec, widgets
          .image(settings:get("Image"))
          :width("fill")
          :height("fill")
          :content_fit("cover")
          :opacity(settings:get("Opacity") / 100)
          :into())
      end

      return spacing.apply(widgets.stack(stack_vec):width("fill"):height("fill"):into())
    end
}
