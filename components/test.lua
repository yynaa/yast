function settings()
  return build_settings():boolean("test bool", false)
end

function widget()
  return widgets.text("hello there"):align_x("right"):align_y("bottom"):color(1.0, 0.0, 0.0, 1.0):size(40.0):width("fixed", 400):height("fill")
end
