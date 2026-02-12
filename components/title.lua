local sizing = require("sizing")
local background = require("background")
local text = require("text")

return {
  ["name"] = "Title",
  ["author"] = "yyna",
  ["settings"] = function()
    return settings_factory()
      :header("Position & Size")
      :plugin(sizing.plugin)
      :plugin(background.plugin)
      :header("Text")
      :plugin(text.plugin("Title", true, false))
      :plugin(text.plugin("Category", true, false))
      :plugin(text.plugin("Attempt Counter", false, false, function(s) return s("Show Attempt Counter") end))
      :header("Information")
      :boolean("Show Attempt Counter", true)
      :boolean("Show Game Icon", true)
    end,
  ["widget"] =
    function()
      local content_column = widgets.column({
        text.write(run.game_name, "Title")
        :width("fill")
        :height("fill_portion", text.size("Title"))
        :align_x(text.align_x("Title"))
        :align_y("center")
        :into(),
        text.write(run.category_name, "Category")
        :width("fill")
        :height("fill_portion", text.size("Category"))
        :align_x(text.align_x("Category"))
        :align_y("center")
        :into()
      }):width("fill"):height("fill"):padding(0, 5, 0, 5):into()
      
      local stack_vec = {}
      
      local img = run.game_icon
      if img and setting("Show Game Icon") then
        table.insert(stack_vec, widgets.container(
          widgets.image(img):height("fill"):into()
        ):padding(5,5,5,5):width("fill"):height("fill"):align_x("left"):align_y("center"):into())
      end
      
      if setting("Show Attempt Counter") then
        table.insert(stack_vec, widgets.container(
          text.write(tostring(run.attempt_count), "Attempt Counter"):width("fill"):height("fill"):align_x("right"):align_y("bottom"):into()
        ):padding(5,5,5,5):width("fill"):height("fill"):align_x("left"):align_y("center"):into())
      end
      
      table.insert(stack_vec, content_column)
      
      return sizing.apply(background.apply(widgets.stack(stack_vec):width("fill"):height("fill"):into()))
    end
}
