local sizing = require("sizing")
local background = require("background")
local time = require("time")

--- @param i number
local function segment_content(i)
  local segment = run.segments[i]
  local segment_comp = segment.comparisons[snapshot.current_comparison]
  local segment_best = segment.comparisons["Best Segments"]
  local analysis_segment = analysis.segments[i]
  local analysis_comp_segment = analysis.comparisons[snapshot.current_comparison].segments[i]
  
  local stack_vec = {}
  local delta_accessor = time.accessor_add(time.accessor_or_zero(analysis_comp_segment.last_delta), analysis_comp_segment.live_segment_delta)
  local time_accessor = segment_comp
  if snapshot.current_split and i < snapshot.current_split then
    delta_accessor = analysis_comp_segment.last_delta
    time_accessor = time.accessor_add(segment_comp, delta_accessor)
  end
  
  local timen = time_accessor.real_time
  local delta = delta_accessor.real_time
  local time_best = segment_best.real_time
  if snapshot.current_timing_method == "GameTime" then
    timen = time_accessor.game_time
    delta = delta_accessor.game_time
    time_best = segment_best.game_time
  end
  
  if not timen or not snapshot.current_split or i > snapshot.current_split or i == snapshot.current_split and timen + delta < time_best then
    delta = nil
  end
  
  if snapshot.current_split and i == snapshot.current_split then
    table.insert(stack_vec, widgets.container(
      widgets.space():width("fill"):height("fill"):into()
    ):width("fill"):height("fill"):style({0,0,0,0},{0,0,1,1}):into())
  end
  
  local delta_color = "Color Ahead"
  if delta ~= nil and delta >= 0 then delta_color = "Color Behind" end
  
  local row_vec = {}
  
  if segment.icon then
    table.insert(row_vec, widgets.image(segment.icon):width("shrink"):height("fill"):into())
  end
  
  table.insert(row_vec, widgets.column({
    -- split name
    widgets.text(segment.name)
      :width("fill"):height("fill"):align_x("left"):align_y("center")
      :style(setting("Color Text"))
      :into(),
    -- times and delta
    widgets.container(
      widgets.row({
        widgets.space():width("fill"):into(),
        widgets.text(time.format_delta(delta, setting("Delta Decimals")))
          :width("shrink"):height("fill"):align_x("right"):align_y("center")
          :style(setting(delta_color))
          :into(),
        widgets.text(time.format(timen, setting("Segment Time Decimals")))
          :width("shrink"):height("fill"):align_x("right"):align_y("center")
          :style(setting("Color Text"))
          :into()
      }):width("fill"):height("fill"):spacing(10.0):into()
    ):width("fill"):height("fill"):into()
  }):width("fill"):height("fill"):into())
  
  table.insert(stack_vec, widgets.row(row_vec):width("fill"):height("fill"):spacing(5.0):padding(5.0, 5.0, 5.0, 5.0):into())
  
  return widgets.stack(stack_vec):width("fill_portion", 1):height("fill"):into()
end

return {
  ["name"] = "Splits",
  ["author"] = "yyna",
  ["settings"] = function()
    return settings_factory()
      :header("Position & Size")
      :plugin(sizing.plugin)
      :plugin(background.plugin)
      :header("Splits")
      :number("Total Splits", 10)
      :number("Upcoming Splits", 1)
      :boolean("Always Show Last Split", true)
      :number_range("Segment Time Decimals", 0, 3, 1, 2)
      :number_range("Delta Decimals", 0, 3, 1, 1)
      :header("Color")
      :color("Color Text",1,1,1,1)
      :color("Color Ahead",0,1,0,1)
      :color("Color Behind",1,0,0,1)
    end,
  ["widget"] =
    function()
      local ts = setting("Total Splits")
      local asls = setting("Always Show Last Split")
      local current_split = snapshot.current_split or 1
      local range_start, range_end
      
      if ts >= #run.segments then
        range_start = 1
        range_end = #run.segments
      else
        range_end = current_split + setting("Upcoming Splits")
        range_start = range_end - setting("Total Splits") + 1
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
