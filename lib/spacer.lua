--- @param settings settings
--- @return settings
local function plugin(settings)
  return settings
    :number_range("Component Width", 100., 1., 100., 1.)
    :options("Component X Align", "Center", {"Left", "Center", "Right"})
    :number_range("Component Height", 100., 1., 100., 1.)
    :options("Component Y Align", "Center", {"Top", "Center", "Bottom"})
end

--- @param widget widget
--- @return widget
local function apply(widget)
  local x_fill = settings:get("Component X Fill")
  local x_align = settings:get("Component X Align")
  local y_fill = settings:get("Component Y Fill")
  local y_align = settings:get("Component Y Align")

  local width_portion = settings:get("Component Width")
  local space_width_portion = 100 - width_portion
  if x_align == "Center" then
    space_width_portion = space_width_portion / 2
  end
  local row_vec = {}
  if x_fill and x_align ~= "Left" and space_width_portion > 0 then
    table.insert(row_vec, widgets.space():height("fill"):width("fill_portion", space_width_portion):into())
  end
  table.insert(row_vec, widgets.container(widget):height("fill"):width("fill_portion", width_portion):into())
  if x_fill and x_align ~= "Right" and space_width_portion > 0 then
    table.insert(row_vec, widgets.space():height("fill"):width("fill_portion", space_width_portion):into())
  end

  local height_portion = settings:get("Component Height")
  local space_height_portion = 100 - height_portion
  if y_align == "Center" then
    space_height_portion = space_height_portion / 2
  end
  local column_vec = {}
  if x_fill and y_align ~= "Top" and space_height_portion > 0 then
    table.insert(column_vec, widgets.space():width("fill"):height("fill_portion", space_height_portion):into())
  end
  table.insert(column_vec, widgets.row(row_vec):width("fill"):height("fill_portion", height_portion):into())
  if x_fill and y_align ~= "Bottom" and space_height_portion > 0 then
    table.insert(column_vec, widgets.space():width("fill"):height("fill_portion", space_height_portion):into())
  end

  return widgets.column(column_vec):width("fill"):height("fill"):into()
end

return {
  ["plugin"] = plugin,
  ["apply"] = apply
}
