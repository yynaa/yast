local sizing = require("sizing")
local background = require("background")
local libtime = require("time")

local function time_or_zero(a)
  local real_time = 0
  local game_time = 0
  if a.real_time then
    real_time = a.real_time
  end
  if a.game_time then
    game_time = a.game_time
  end
  return {
    ["real_time"] = real_time,
    ["game_time"] = game_time,
  }
end

local function time_adder(a,b) 
  local real_time = nil
  local game_time = nil
  if a.real_time and b.real_time then
    real_time = a.real_time + b.real_time
  end
  if a.game_time and b.game_time then
    game_time = a.game_time + b.game_time
  end
  return {
    ["real_time"] = real_time,
    ["game_time"] = game_time,
  }
end

--- @param i number
local function segment_content(i)
  local segment = run.segments[i]
  local segment_comp = segment.comparisons[snapshot.current_comparison]
  local segment_best = segment.comparisons["Best Segments"]
  local analysis_segment = analysis.segments[i]
  local analysis_comp_segment = analysis.comparisons[snapshot.current_comparison].segments[i]
  
  local stack_vec = {}
  local delta_accessor = time_adder(time_or_zero(analysis_comp_segment.last_delta), analysis_comp_segment.live_segment_delta)
  local time_accessor = segment_comp
  if snapshot.current_split and i < snapshot.current_split then
    delta_accessor = analysis_comp_segment.last_delta
    time_accessor = time_adder(segment_comp, delta_accessor)
  end
  
  local time = time_accessor.real_time
  local delta = delta_accessor.real_time
  local time_best = segment_best.real_time
  if snapshot.current_timing_method == "GameTime" then
    time = time_accessor.game_time
    delta = delta_accessor.game_time
    time_best = segment_best.game_time
  end
  
  if not time or not snapshot.current_split or i > snapshot.current_split or i == snapshot.current_split and time + delta < time_best then
    delta = nil
  end
  
  if snapshot.current_split and i == snapshot.current_split then
    table.insert(stack_vec, widgets.container(
      widgets.space():width("fill"):height("fill"):into()
    ):width("fill"):height("fill"):style({0,0,0,0},{0,0,1,1}):into())
  end
  
  table.insert(stack_vec, widgets.column({
    -- split name
    widgets.text(segment.name)
      :width("fill"):height("fill"):align_x("left"):align_y("center")
      :into(),
    -- times and delta
    widgets.container(
      widgets.row({
        widgets.space():width("fill"):into(),
        widgets.text(libtime.format_delta(delta, settings:get("Delta Decimals")))
          :width("shrink"):height("fill"):align_x("right"):align_y("center")
          :into(),
        widgets.text(libtime.format(time, settings:get("Segment Time Decimals")))
          :width("shrink"):height("fill"):align_x("right"):align_y("center")
          :into()
      }):width("fill"):height("fill"):spacing(10.0):into()
    ):width("fill"):height("fill"):into()
  }):width("fill"):height("fill"):padding(5.0, 5.0, 5.0, 5.0):into())
  
  return widgets.stack(stack_vec):width("fill_portion", 1):height("fill"):into()
end

return {
  ["name"] = "Splits",
  ["author"] = "yyna",
  ["settings"] = build_settings()
    :plugin(sizing.plugin)
    :plugin(background.plugin)
    :number("Total Splits", 10)
    :number("Upcoming Splits", 1)
    :boolean("Always Show Last Split", true)
    :number_range("Segment Time Decimals", 2, 0, 3, 1)
    :number_range("Delta Decimals", 1, 0, 3, 1),
  ["widget"] =
    function()
      local ts = settings:get("Total Splits")
      local asls = settings:get("Always Show Last Split")
      local current_split = snapshot.current_split or 1
      local range_start, range_end
      
      if ts >= #run.segments then
        range_start = 1
        range_end = #run.segments
      else
        range_end = current_split + settings:get("Upcoming Splits")
        range_start = range_end - settings:get("Total Splits") + 1
        if asls then range_start = range_start + 1 end
        if range_start < 1 then
          range_end = range_end + (1 - range_start)
          range_start = 1
        end
        local upper_bound = #run.segments
        if asls then upper_bound = upper_bound - 1 end
        if range_end > upper_bound then
          range_start = range_start - (range_end - upper_bound)
          range_end = upper_bound
        end
      end
      
      local column_vec = {}
      for i = range_start, range_end do
        table.insert(column_vec, segment_content(i))
      end
      if asls and ts < #run.segments then
        table.insert(column_vec, segment_content(#run.segments))
      end
      
      return sizing.apply(background.apply(
        widgets.column(column_vec):width("fill"):height("fill"):into()
      ))
    end
}
