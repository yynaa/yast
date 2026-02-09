--- @param settings settings
--- @return settings
local function plugin(settings)
  return settings
    :options("Background Type", "None", {"None", "Solid", "Image"})
    :color("Background: Solid Color",1, 1, 1, 1)
    :options("Background: Gradient Direction", "Vertical", {"Vertical", "Horizontal"})
    :color("Background: Gradient Color 1", 1, 1, 1, 1)
    :color("Background: Gradient Color 2", 1, 1, 1, 1)
    :image("Background: Image")
end

--- @param widget widget
--- @return widget
local function apply(widget)
  local background_type = settings:get("Background Type")
  local image = settings:get("Background: Image")
  
  if background_type == "Solid" then
    return widgets.container(widget)
      :style({0,0,0,0},settings:get("Background: Solid Color"))
      :width("fill"):height("fill"):into()
  elseif background_type == "Image" and image then
    return widgets.stack({
      widgets.image(image):width("fill"):height("fill"):content_fit("cover"):into(),
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
