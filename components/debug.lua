return {
  ["name"] = "Debug",
  ["author"] = "yyna",
  ["settings"] = build_settings()
      :boolean("show_snapshot", true)
      :boolean("show_run", true)
      :boolean("show_analysis", true)
      :boolean("show_details", false)
      :string("prefix", "Debug")
      :options("view_mode", "Simple", {"Simple", "Detailed", "Compact"})
      :number("decimal_places", 2)
      :number_range("font_size", 12, 8, 32, 1)
      :color("text_color", 1.0, 1.0, 1.0, 1.0)
      :image("background"),
  ["widget"] =
    function()
      local debugs = {}
      if settings:get("show_snapshot") then
        table.insert(debugs, widgets.text("current_attempt_duration = " .. snapshot.current_attempt_duration):into())
        table.insert(debugs, widgets.text("current_comparison = " .. snapshot.current_comparison):into())
        table.insert(debugs, widgets.text("current_phase = " .. snapshot.current_phase):into())
        table.insert(debugs, widgets.text("current_split = " .. tostring(snapshot.current_split)):into())
        table.insert(debugs, widgets.text("current_timing_method = " .. snapshot.current_timing_method):into())
        table.insert(debugs, widgets.text("current_time.real_time = " .. tostring(snapshot.current_time.real_time)):into())
        table.insert(debugs, widgets.text("current_time.game_time = " .. tostring(snapshot.current_time.game_time)):into())
      end
      if settings:get("show_run") then
        table.insert(debugs, widgets.text("run.game_name = " .. tostring(run.game_name)):into())
        table.insert(debugs, widgets.text("run.game_icon = " .. tostring(run.game_icon and #run.game_icon or 0) .. " bytes"):into())
        table.insert(debugs, widgets.text("run.category_name = " .. tostring(run.category_name)):into())
        table.insert(debugs, widgets.text("run.attempt_count = " .. tostring(run.attempt_count)):into())
        table.insert(debugs, widgets.text("run.metadata.run_id = " .. tostring(run.metadata.run_id)):into())
        table.insert(debugs, widgets.text("run.metadata.platform_name = " .. tostring(run.metadata.platform_name)):into())
        table.insert(debugs, widgets.text("run.metadata.uses_emulator = " .. tostring(run.metadata.uses_emulator)):into())
        table.insert(debugs, widgets.text("run.metadata.region_name = " .. tostring(run.metadata.region_name)):into())
        if settings:get("show_details") then
          for i, seg in ipairs(run.segments) do
            table.insert(debugs, widgets.text("run.segments[" .. i .. "].name = " .. tostring(seg.name)):into())
            table.insert(debugs, widgets.text("run.segments[" .. i .. "].icon = " .. tostring(seg.icon and #seg.icon or 0) .. " bytes"):into())
            for comp_name, comp_time in pairs(seg.comparisons) do
              table.insert(debugs, widgets.text("  run.segments[" .. i .. "].comparisons." .. comp_name .. ".real_time = " .. tostring(comp_time.real_time)):into())
              table.insert(debugs, widgets.text("  run.segments[" .. i .. "].comparisons." .. comp_name .. ".game_time = " .. tostring(comp_time.game_time)):into())
            end
          end
        end
      end

      if settings:get("show_analysis") then
        table.insert(debugs, widgets.text("analysis.live_delta.real_time = " .. tostring(analysis.live_delta.real_time)):into())
        table.insert(debugs, widgets.text("analysis.live_delta.game_time = " .. tostring(analysis.live_delta.game_time)):into())
        table.insert(debugs, widgets.text("analysis.live_split_delta.real_time = " .. tostring(analysis.live_split_delta.real_time)):into())
        table.insert(debugs, widgets.text("analysis.live_split_delta.game_time = " .. tostring(analysis.live_split_delta.game_time)):into())
  
        for comp_name, comp_data in pairs(analysis.comparisons) do
          table.insert(debugs, widgets.text("analysis.comparisons." .. comp_name .. ".current_pace.time = " .. tostring(comp_data.current_pace.time)):into())
          table.insert(debugs, widgets.text("analysis.comparisons." .. comp_name .. ".current_pace.is_live = " .. tostring(comp_data.current_pace.is_live)):into())
          table.insert(debugs, widgets.text("analysis.comparisons." .. comp_name .. ".delta.delta = " .. tostring(comp_data.delta.delta)):into())
          table.insert(debugs, widgets.text("analysis.comparisons." .. comp_name .. ".delta.is_live = " .. tostring(comp_data.delta.is_live)):into())
          for i, seg_data in ipairs(comp_data.segments) do
            table.insert(debugs, widgets.text("  analysis.comparisons." .. comp_name .. ".segments[" .. i .. "].possible_save_time.time = " .. tostring(seg_data.possible_save_time.time)):into())
            table.insert(debugs, widgets.text("  analysis.comparisons." .. comp_name .. ".segments[" .. i .. "].possible_save_time.is_live = " .. tostring(seg_data.possible_save_time.is_live)):into())
            table.insert(debugs, widgets.text("  analysis.comparisons." .. comp_name .. ".segments[" .. i .. "].total_possible_save_time.time = " .. tostring(seg_data.total_possible_save_time.time)):into())
            table.insert(debugs, widgets.text("  analysis.comparisons." .. comp_name .. ".segments[" .. i .. "].total_possible_save_time.is_live = " .. tostring(seg_data.total_possible_save_time.is_live)):into())
            table.insert(debugs, widgets.text("  analysis.comparisons." .. comp_name .. ".segments[" .. i .. "].is_best_segment.real_time = " .. tostring(seg_data.is_best_segment.real_time)):into())
            table.insert(debugs, widgets.text("  analysis.comparisons." .. comp_name .. ".segments[" .. i .. "].is_best_segment.game_time = " .. tostring(seg_data.is_best_segment.game_time)):into())
            table.insert(debugs, widgets.text("  analysis.comparisons." .. comp_name .. ".segments[" .. i .. "].last_delta.real_time = " .. tostring(seg_data.last_delta.real_time)):into())
            table.insert(debugs, widgets.text("  analysis.comparisons." .. comp_name .. ".segments[" .. i .. "].last_delta.game_time = " .. tostring(seg_data.last_delta.game_time)):into())
            table.insert(debugs, widgets.text("  analysis.comparisons." .. comp_name .. ".segments[" .. i .. "].live_segment_delta.real_time = " .. tostring(seg_data.live_segment_delta.real_time)):into())
            table.insert(debugs, widgets.text("  analysis.comparisons." .. comp_name .. ".segments[" .. i .. "].live_segment_delta.game_time = " .. tostring(seg_data.live_segment_delta.game_time)):into())
            table.insert(debugs, widgets.text("  analysis.comparisons." .. comp_name .. ".segments[" .. i .. "].previous_segment_delta.real_time = " .. tostring(seg_data.previous_segment_delta.real_time)):into())
            table.insert(debugs, widgets.text("  analysis.comparisons." .. comp_name .. ".segments[" .. i .. "].previous_segment_delta.game_time = " .. tostring(seg_data.previous_segment_delta.game_time)):into())
          end
        end
  
        for i, seg_timing in ipairs(analysis.segments) do
          table.insert(debugs, widgets.text("analysis.segments[" .. i .. "].live_segment_time.real_time = " .. tostring(seg_timing.live_segment_time.real_time)):into())
          table.insert(debugs, widgets.text("analysis.segments[" .. i .. "].live_segment_time.game_time = " .. tostring(seg_timing.live_segment_time.game_time)):into())
          table.insert(debugs, widgets.text("analysis.segments[" .. i .. "].previous_segment_time.real_time = " .. tostring(seg_timing.previous_segment_time.real_time)):into())
          table.insert(debugs, widgets.text("analysis.segments[" .. i .. "].previous_segment_time.game_time = " .. tostring(seg_timing.previous_segment_time.game_time)):into())
        end
  
        table.insert(debugs, widgets.text("analysis.pb_chance.chance = " .. tostring(analysis.pb_chance.chance)):into())
        table.insert(debugs, widgets.text("analysis.pb_chance.is_live = " .. tostring(analysis.pb_chance.is_live)):into())
        table.insert(debugs, widgets.text("analysis.sum_of_best_segments.real_time = " .. tostring(analysis.sum_of_best_segments.real_time)):into())
        table.insert(debugs, widgets.text("analysis.sum_of_best_segments.game_time = " .. tostring(analysis.sum_of_best_segments.game_time)):into())
        table.insert(debugs, widgets.text("analysis.sum_of_worst_segments.real_time = " .. tostring(analysis.sum_of_worst_segments.real_time)):into())
        table.insert(debugs, widgets.text("analysis.sum_of_worst_segments.game_time = " .. tostring(analysis.sum_of_worst_segments.game_time)):into())
        table.insert(debugs, widgets.text("analysis.total_playtime = " .. tostring(analysis.total_playtime)):into())
      end

      local result = widgets.column(debugs):into()
      local img = settings:get("background")

      if img ~= nil then
        local vec = {}
        table.insert(vec, widgets.image(img):width("fill"):height("fill"):into())
        table.insert(vec, result)
        result = widgets.stack(vec):width("fill"):height("fill"):into()
      end

      return result
    end
}
