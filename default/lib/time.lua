local function prepend_zeros(n, l)
	local tn = tostring(n)
	local zta = l - #tn
	for i = 1, zta do
		tn = "0" .. tn
	end
	return tn
end

--- @param time number|nil
--- @param decimals number
--- @return string
local function format(time, decimals)
	if time == nil then
		return "-"
	end

	local leading_zeros_seconds = 2
	if time < 10 then
		leading_zeros_seconds = 1
	end
	local leading_zeros_minutes = 2
	if time < 10 * 60 then
		leading_zeros_minutes = 1
	end

	local time_hours = math.floor(time / (60 * 60)) % (60 * 60)
	local time_minutes = math.floor(time / 60) % 60
	local time_seconds = math.floor(time) % 60
	local time_ms = math.floor(time * math.floor(10 ^ decimals)) % math.floor(10 ^ decimals)

	local time_string_hours = tostring(time_hours)
	local time_string_minutes = prepend_zeros(time_minutes, leading_zeros_minutes)
	local time_string_seconds = prepend_zeros(time_seconds, leading_zeros_seconds)
	local time_string_ms = prepend_zeros(time_ms, decimals)

	local time_string = time_string_seconds
	if decimals > 0 then
		time_string = time_string .. "." .. time_string_ms
	end
	if time_hours ~= 0 or time_minutes ~= 0 then
		time_string = time_string_minutes .. ":" .. time_string
	end
	if time_hours ~= 0 then
		time_string = time_string_hours .. ":" .. time_string
	end

	return time_string
end

--- @param delta number|nil
--- @param decimals number
--- @return string
local function format_delta(delta, decimals)
	if delta == nil then
		return ""
	end

	local time_string = format(math.abs(delta), decimals)
	if delta >= 0 then
		time_string = "+" .. time_string
	else
		time_string = "-" .. time_string
	end

	return time_string
end

--- @param accessor table
--- @param timing_method string|nil
--- @return number
local function timing_accessor(accessor, timing_method)
	if timing_method == nil then
		timing_method = snapshot.current_timing_method
	end
	if timing_method == "GameTime" then
		return accessor.game_time
	else
		return accessor.real_time
	end
end

local function accessor_zero(a)
	return {
		["real_time"] = 0,
		["game_time"] = 0,
	}
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

local function accessor_add(a, b)
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

local function accessor_sub(a, b)
	local real_time = nil
	local game_time = nil
	if a.real_time and b.real_time then
		real_time = a.real_time - b.real_time
	end
	if a.game_time and b.game_time then
		game_time = a.game_time - b.game_time
	end
	return {
		["real_time"] = real_time,
		["game_time"] = game_time,
	}
end

--- @param timing_method string|nil
--- @return number|nil
local function current_time(timing_method)
	return timing_accessor(snapshot.current_time, timing_method)
end

--- @param segment number|nil
--- @param comparison string|nil
--- @param timing_method string|nil
--- @return number|nil
local function live_delta(segment, comparison, timing_method)
	if segment == nil then
		segment = snapshot.current_split
	end
	if segment == nil then
		return nil
	end
	if not comparison then
		comparison = snapshot.current_comparison
	end
	local current_split = snapshot.current_split
	if current_split ~= nil then
		local analysis_comp_segment = analysis.comparisons[comparison].segments[segment]
		if current_split == segment then
			return timing_accessor(
				accessor_add(
					accessor_or_zero(analysis_comp_segment.last_delta),
					analysis_comp_segment.live_segment_delta
				),
				timing_method
			)
		elseif segment < current_split then
			return timing_accessor(analysis_comp_segment.last_delta, timing_method)
		else
			return nil
		end
	end
	return nil
end

--- @param segment number|nil
--- @param comparison string|nil
--- @param timing_method string|nil
--- @return number|nil
local function live_segment_delta(segment, comparison, timing_method)
	if segment == nil then
		segment = snapshot.current_split
	end
	if segment == nil then
		return nil
	end
	if not comparison then
		comparison = snapshot.current_comparison
	end
	local current_split = snapshot.current_split
	if current_split ~= nil then
		local analysis_comp_segment = analysis.comparisons[comparison].segments[segment]
		if current_split == segment then
			return timing_accessor(analysis_comp_segment.live_segment_delta, timing_method)
		elseif current_split > segment then
			if segment > 1 then
				return timing_accessor(
					accessor_sub(
						analysis_comp_segment.last_delta,
						analysis.comparisons[comparison].segments[segment - 1].last_delta
					),
					timing_method
				)
			else
				return timing_accessor(analysis_comp_segment.last_delta, timing_method)
			end
		else
			return nil
		end
	end
	return nil
end

--- @param segment number|nil
--- @param comparison string|nil
--- @param timing_method string|nil
--- @return number|nil
local function live_split_time(segment, comparison, timing_method)
	if segment == nil then
		segment = snapshot.current_split
	end
	if segment == nil then
		return nil
	end
	if not comparison then
		comparison = snapshot.current_comparison
	end
	local current_split = snapshot.current_split
	local run_segment = run.segments[segment]
	local segment_comp = run_segment.comparisons[comparison]
	if current_split ~= nil then
		if current_split >= segment then
			return timing_accessor(segment_comp, timing_method)
		else
			return timing_accessor(segment_comp, timing_method) + (live_delta(segment, comparison, timing_method) or 0)
		end
	end
	return timing_accessor(run_segment.comparisons[comparison], timing_method)
end

--- @param segment number|nil
--- @param comparison string|nil
--- @param timing_method string|nil
--- @return number|nil
local function live_segment_time(segment, comparison, timing_method)
	if segment == nil then
		segment = snapshot.current_split
	end
	if segment == nil then
		return nil
	end
	if not comparison then
		comparison = snapshot.current_comparison
	end
	local current_split = snapshot.current_split
	local segment_comp = run.segments[segment].comparisons[comparison]
	local a = timing_accessor(segment_comp, timing_method)
	if segment > 1 then
		a = timing_accessor(
			accessor_sub(segment_comp, run.segments[segment - 1].comparisons[comparison]),
			timing_method
		)
	end
	if current_split ~= nil then
		if current_split >= segment then
			return a
		else
			return a + (live_segment_delta(segment, comparison, timing_method) or 0)
		end
	end
	return a
end

--- @param timing_method string|nil
--- @return number|nil
local function sum_of_best(timing_method)
	return timing_accessor(analysis.sum_of_best_segments, timing_method)
end

--- @param timing_method string|nil
--- @return number|nil
local function sum_of_worst(timing_method)
	return timing_accessor(analysis.sum_of_worst_segments, timing_method)
end

--- @param segment number|nil
--- @param comparison string|nil
--- @param timing_method string|nil
--- @return number|nil
local function possible_time_save(segment, comparison, timing_method)
	if segment == nil then
		segment = snapshot.current_split
	end
	if segment == nil then
		return nil
	end
	if not comparison then
		comparison = snapshot.current_comparison
	end
	local best_segment_time = live_segment_time(segment, "Best Segments", timing_method)
	local comp_segment_time = live_segment_time(segment, comparison, timing_method)
	return comp_segment_time - best_segment_time
end

--- @param comparison string|nil
--- @param timing_method string|nil
--- @return number|nil
local function total_possible_time_save(comparison, timing_method)
	if not comparison then
		comparison = snapshot.current_comparison
	end
	local segment = snapshot.current_split
	if segment ~= nil then
		local best_time = live_split_time(#run.segments, "Best Segments", timing_method)
			- live_split_time(segment, "Best Segments", timing_method)
		local comp_time = live_segment_time(#run.segments, comparison, timing_method)
			- live_split_time(segment, comparison, timing_method)
		return comp_time - best_time
	else
		local best_time = live_split_time(#run.segments, "Best Segments", timing_method)
		local comp_time = live_split_time(#run.segments, comparison, timing_method)
		if best_time == nil or comp_time == nil then
			return nil
		end
		return comp_time - best_time
	end
end

return {
	format = format,
	format_delta = format_delta,
	current_time = current_time,
	live_delta = live_delta,
	live_segment_delta = live_segment_delta,
	live_split_time = live_split_time,
	live_segment_time = live_segment_time,
	sum_of_best = sum_of_best,
	sum_of_worst = sum_of_worst,
	possible_time_save = possible_time_save,
	total_possible_time_save = total_possible_time_save,
}
