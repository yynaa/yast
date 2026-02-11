return {
  ["name"] = "Title",
  ["author"] = "yyna",
  ["settings"] = 
    function()
      return settings_factory():boolean("show", false):header("hello!!!", function(s) return s("show") end)
    end,
  ["widget"] =
    function()
      return widgets.space():into()
    end
}
