return {
  ["name"] = "Awesome Component",
  ["author"] = "yyna",
  ["settings"] = build_settings():boolean("Awesome", false),
  ["widget"] =
    function()
      return widgets
        .text(settings:get("Awesome") and "I am so awesome!" or "Not cool...")
        :width("fill")
        :height("fill")
        :align_x("center")
        :align_y("center")
        :into()
    end
}
