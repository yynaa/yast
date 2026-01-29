--- Test Component
--- yyna

function settings()
  return build_settings():boolean("test bool", false)
end

function widget()
  return widgets.text("hello there"):into()
end
