--- @param settings settings_factory
--- @return settings_factory
local function plugin(settings)
  return settings
    :options("Background: Type", {"None", "Solid", "Image"}, "None")
    :color("Background: Solid Color", 1, 1, 1, 1, function(s) return s("Background: Type") == "Solid" end)
    :image("Background: Image", function(s) return s("Background: Type") == "Image" end)
    :number_range("Background: Image Opacity", 0, 100, 1, 0, function(s) return s("Background: Type") == "Image" end)
end

--- @param widget widget
--- @return widget
local function apply(widget)
  local background_type = setting("Background: Type")
  local image = setting("Background: Image")
  
  if background_type == "Solid" then
    return widgets.container(widget)
      :style({0,0,0,0},setting("Background: Solid Color"))
      :width("fill"):height("fill"):into()
  elseif background_type == "Image" and image then
    return widgets.stack({
      widgets.image(image):width("fill"):height("fill"):content_fit("cover"):opacity(setting("Background: Image Opacity") / 100):into(),
      widget
    }):width("fill"):height("fill"):into()
  else
    return widget
  end
end

return {
  ["plugin"] = plugin,
  ["apply"] = apply
}
