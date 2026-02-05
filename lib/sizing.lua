--- @param settings settings
--- @return settings
local function plugin(settings)
  return settings
    :number("Component Width", 100.)
    :boolean("Component Fixed Width", false)
    :number("Component Height", 100.)
    :boolean("Component Fixed Height", false)
end

--- @return string, number
local function get_width()
  local typ = "fill_portion"
  if settings:get("Component Fixed Width") then typ = "fixed" end
  return typ, settings:get("Component Width")
end

--- @return string, number
local function get_height()
  local typ = "fill_portion"
  if settings:get("Component Fixed Height") then typ = "fixed" end
  return typ, settings:get("Component Height")
end

return {
  ["plugin"] = plugin,
  ["get_width"] = get_width,
  ["get_height"] = get_height
}
