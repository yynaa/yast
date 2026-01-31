return {
  ["name"] = "Debug",
  ["author"] = "Built-in",
  ["settings"] = build_settings(),
  ["widget"] =
    function()
      local debugs = {}
      table.insert(debugs, widgets.text("current_attempt_duration = " .. snapshot.current_attempt_duration):into())
      table.insert(debugs, widgets.text("current_comparison = " .. snapshot.current_comparison):into())
      table.insert(debugs, widgets.text("current_phase = " .. snapshot.current_phase):into())
      table.insert(debugs, widgets.text("current_split = " .. tostring(snapshot.current_split)):into())
      table.insert(debugs, widgets.text("current_timing_method = " .. snapshot.current_timing_method):into())
      table.insert(debugs, widgets.text("current_time.real_time = " .. tostring(snapshot.current_time.real_time)):into())
      table.insert(debugs, widgets.text("current_time.game_time = " .. tostring(snapshot.current_time.game_time)):into())
      
      return widgets
        .column(debugs)
        :into()
    end
}
