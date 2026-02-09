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

return {
  ["format"] = format,
  ["format_delta"] = format_delta
}
