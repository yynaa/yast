--- @param settings settings_factory
--- @return settings_factory
local function plugin(settings)
  return settings
    :number("Sizing: Component Width", 100.)
    :boolean("Sizing: Component Fixed Width", false)
    :number("Sizing: Component Height", 100.)
    :boolean("Sizing: Component Fixed Height", false)
end

--- @return string, number
local function get_width()
  local typ = "fill_portion"
  if setting("Sizing: Component Fixed Width") then typ = "fixed" end
  return typ, setting("Sizing: Component Width")
end

--- @return string, number
local function get_height()
  local typ = "fill_portion"
  if setting("Sizing: Component Fixed Height") then typ = "fixed" end
  return typ, setting("Sizing: Component Height")
end

local function apply(widget)
  return widgets.container(widget):width(get_width()):height(get_height()):into()
end

return {
  ["plugin"] = plugin,
  ["get_width"] = get_width,
  ["get_height"] = get_height,
  ["apply"] = apply
}
