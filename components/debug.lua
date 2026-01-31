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
      table.insert(debugs, widgets.text("run.game_name = " .. tostring(run.game_name)):into())
      table.insert(debugs, widgets.text("run.game_icon = " .. tostring(#run.game_icon) .. " bytes"):into())
      table.insert(debugs, widgets.text("run.category_name = " .. tostring(run.category_name)):into())
      table.insert(debugs, widgets.text("run.attempt_count = " .. tostring(run.attempt_count)):into())
      table.insert(debugs, widgets.text("run.metadata.run_id = " .. tostring(run.metadata.run_id)):into())
      table.insert(debugs, widgets.text("run.metadata.platform_name = " .. tostring(run.metadata.platform_name)):into())
      table.insert(debugs, widgets.text("run.metadata.uses_emulator = " .. tostring(run.metadata.uses_emulator)):into())
      table.insert(debugs, widgets.text("run.metadata.region_name = " .. tostring(run.metadata.region_name)):into())
      for i, seg in ipairs(run.segments) do
        table.insert(debugs, widgets.text("run.segments[" .. i .. "].name = " .. tostring(seg.name)):into())
        table.insert(debugs, widgets.text("run.segments[" .. i .. "].icon = " .. tostring(#seg.icon) .. " bytes"):into())
        for comp_name, comp_time in pairs(seg.comparisons) do
          table.insert(debugs, widgets.text("  run.segments[" .. i .. "].comparisons." .. comp_name .. ".real_time = " .. tostring(comp_time.real_time)):into())
          table.insert(debugs, widgets.text("  run.segments[" .. i .. "].comparisons." .. comp_name .. ".game_time = " .. tostring(comp_time.game_time)):into())
        end
      end

      return widgets
        .column(debugs)
        :into()
    end
}
