local background = require("background")
local sizing = require("sizing")
local text = require("text")

--- @param label_text string
--- @return fun(settings: settings_factory): settings_factory
local function plugin(label_text)
  return function(settings)
    local show_if_label = function(s) return s("Show Label") end
    
    return settings
      :header("Position & Size")
      :plugin(background.plugin)
      :plugin(sizing.plugin)
      :header("Label")
      :boolean("Show Label", true)
      :string("Label Text: Content", label_text, show_if_label)
      :plugin(text.plugin("Label", false, false, show_if_label))
      :header("Value")
      :string("Value Text: Font", "")
      :number("Value Text: Size", 12)
  end
end

--- @param widget_text widget_text
--- @return widget
local function apply(widget_text)
  local modified = widget_text:font(setting("Value Text: Font")):size(setting("Value Text: Size"))
  
  local content
  
  if setting("Show Label") then
    content = widgets.row({
      text.write(setting("Label Text: Content"), "Label"):width("shrink"):height("fill"):align_x("left"):align_y("center"):into(),
      widgets.space():width("fill"):into(),
      modified:width("shrink"):height("fill"):align_x("right"):align_y("center"):into()
    }):padding(5,5,5,5):spacing(5):width("fill"):height("fill"):align_y("center"):into()
  else
    content = modified:width("fill"):height("fill"):align_x("center"):align_y("center"):into()
  end
  
  return sizing.apply(background.apply(content))
end

return {
  ["plugin"] = plugin,
  ["apply"] = apply,
}
