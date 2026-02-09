--- @param settings settings
--- @return settings
local function plugin(settings)
  return settings
    :string("Font Name", "Arial")
    :number("Font Size", 12)
    :color("Text Color", 1,1,1,1)
end

--- @return string
local function font()
  return settings:get("Font Name")
end

--- @return number
local function size()
  return settings:get("Font Size")
end

--- @return number[]
local function color()
  return settings:get("Text Color")
end

return {
  ["plugin"] = plugin,
  ["font"] = font,
  ["size"] = size,
  ["color"] = color
}
