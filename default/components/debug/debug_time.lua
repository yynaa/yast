local time = require "time"

return {
  ["name"] = "Debug: Settings",
  ["author"] = "yyna",
  ["settings"] =
    function()
      return settings_factory()
    end,
  ["widget"] =
    function()
      local debugs = {}
      
      table.insert(debugs, widgets.text("current_time = " .. tostring(time.current_time())):into())
      
      for segment_index, segment in ipairs(run.segments) do
        table.insert(debugs, widgets.text("segment " .. segment.name):into())
        table.insert(debugs, widgets.text("  live_delta = " .. tostring(time.live_delta(segment_index))):into())
        table.insert(debugs, widgets.text("  live_segment_delta = " .. tostring(time.live_segment_delta(segment_index))):into())
        table.insert(debugs, widgets.text("  live_split_time = " .. tostring(time.live_split_time(segment_index))):into())
        table.insert(debugs, widgets.text("  live_segment_time = " .. tostring(time.live_segment_time(segment_index))):into())
      end
      
      return widgets.column(debugs):width("fill"):height("fill"):into()
    end
}
