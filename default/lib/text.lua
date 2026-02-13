--- @param text_name string
--- @param align_x boolean
--- @param align_y boolean
--- @param show_if show_if
--- @return fun(settings: settings_factory): settings_factory
local function plugin(text_name, align_x, align_y, show_if)
  return function(settings)
    local s = settings
      :string(text_name .. " Text: Font", "", show_if)
      :number(text_name .. " Text: Size", 12, show_if)
      :color(text_name .. " Text: Color", 1, 1, 1, 1, show_if)
    
    if align_x then
      s = s:options(text_name .. " Text: Align X", {"Left", "Center", "Right"}, "Center", show_if)
    end
    if align_y then
      s = s:options(text_name .. " Text: Align Y", {"Top", "Center", "Bottom"}, "Center", show_if)
    end
    
    return s
  end
end

--- @param text_name string
--- @return string
local function font(text_name)
  return setting(text_name .. " Text: Font")
end

--- @param text_name string
--- @return number
local function size(text_name)
  return setting(text_name .. " Text: Size")
end

--- @param text_name string
--- @return number[]
local function color(text_name)
  return setting(text_name .. " Text: Color")
end

--- @param text_name string
--- @return string
local function align_x(text_name)
  return string.lower(setting(text_name .. " Text: Align X"))
end

--- @param text_name string
--- @return string
local function align_y(text_name)
  return string.lower(setting(text_name .. " Text: Align Y"))
end

--- @param text string
--- @param text_name string
--- @return widget_text
local function write(text, text_name)
  return widgets.text(text)
    :font(font(text_name))
    :size(size(text_name))
    :style(color(text_name))
end

return {
  ["plugin"] = plugin,
  ["write"] = write,
  ["font"] = font,
  ["size"] = size,
  ["color"] = color,
  ["align_x"] = align_x,
  ["align_y"] = align_y,
}
