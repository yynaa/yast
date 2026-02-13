return {
  ["name"] = "Debug: Settings",
  ["author"] = "yyna",
  ["settings"] =
    function()
      return settings_factory()
        :header("settings")
        :boolean("show_egg", false)
        :string("prefix", "Debug")
        :options("view_mode", {"Simple", "Detailed", "Compact"}, "Simple")
        :number("decimal_places", 2)
        :number_range("font_size", 8, 32, 1, 12)
        :color("text_color", 1.0, 1.0, 1.0, 1.0)
        :image("background")
        :header("egg!", function(s) return s("show_egg") end)
    end,
  ["widget"] =
    function()
      local img = setting("background")
      
      if img then
        return widgets.image(img):width("fill"):height("fill"):into()
      end
      return widgets.space():into()
    end
}
