return {
  ["name"] = "Timer",
  ["author"] = "Built-in",
  ["settings"] = build_settings(),
  ["widget"] =
    function()
      return widgets
        .text(tostring(snapshot.current_time.real_time))
        :width("fill")
        :height("fill")
        :align_x("center")
        :align_y("center")
        :into()
    end
}
