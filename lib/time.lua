local function prepend_zeros(n, l)
  local tn = tostring(n)
  local zta = l - #tn
  for i = 1,zta do
    tn = "0" .. tn
  end
  return tn
end

--- @param time number|nil
--- @param decimals number
--- @return string
local function format(time, decimals)
  if time == nil then return "-" end
  
  local leading_zeros_seconds = 2
  if time < 10 then leading_zeros_seconds = 1 end
  local leading_zeros_minutes = 2
  if time < 10*60 then leading_zeros_minutes = 1 end
  
  local time_hours = math.floor(time/(60*60))%(60*60)
  local time_minutes = math.floor(time/60)%60
  local time_seconds = math.floor(time)%60
  local time_ms = math.floor(time*math.floor(10^decimals))%math.floor(10^decimals)
  
  local time_string_hours = tostring(time_hours)
  local time_string_minutes = prepend_zeros(time_minutes, leading_zeros_minutes)
  local time_string_seconds = prepend_zeros(time_seconds, leading_zeros_seconds)
  local time_string_ms = prepend_zeros(time_ms, decimals)
  
  local time_string = time_string_seconds
  if decimals > 0 then
    time_string = time_string .. "." .. time_string_ms
  end
  if time_hours ~= 0 then
    time_string = time_string_hours .. ":" .. time_string
  end
  if time_hours ~= 0 or time_minutes ~= 0 then
    time_string = time_string_minutes .. ":" .. time_string
  end
  
  return time_string
end

--- @param delta number|nil
--- @param decimals number
--- @return string
local function format_delta(delta, decimals)
  if delta == nil then return "" end
  
  local time_string = format(math.abs(delta), decimals)
  if delta >= 0 then
    time_string = "+" .. time_string
  else
    time_string = "-" .. time_string
  end
  
  return time_string
end

--- @param accessor table
--- @return number
local function current_timing_accessor(accessor)
  if snapshot.current_timing_method == "GameTime" then
    return accessor.game_time
  else
    return accessor.real_time
  end
end


local function accessor_or_zero(a)
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

local function accessor_add(a,b) 
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

--- @return table|nil
local function live_delta()
  local current_split = snapshot.current_split
  if current_split == nil or current_split > #analysis.comparisons[snapshot.current_comparison].segments then return nil end
  local analysis_comp_segment = analysis.comparisons[snapshot.current_comparison].segments[current_split]
  return accessor_add(accessor_or_zero(analysis_comp_segment.last_delta), analysis_comp_segment.live_segment_delta)
end

return {
  ["format"] = format,
  ["format_delta"] = format_delta,
  ["current_timing_accessor"] = current_timing_accessor,
  ["cta"] = current_timing_accessor,
  ["accessor_add"] = accessor_add,
  ["accessor_or_zero"] = accessor_or_zero,
  ["live_delta"] = live_delta
}
