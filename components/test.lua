--- Test Component
--- yyna

function settings()
  return build_settings():boolean("test bool", false)
end

function widget()
  return widgets.text("hello there"):color(1.0,0.0,0.0,1.0):width("fill"):align_x("center"):into()
end
