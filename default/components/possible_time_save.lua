local labeled = require("labeled")
local time = require("time")

local function get(a)
  return time.accessor_sub(run.segments[a].comparisons[snapshot.current_comparison], run.segments[a].comparisons["Best Segments"])
end

local function difference(a,b)
  local a_diff = time.accessor_sub(run.segments[a].comparisons[snapshot.current_comparison], run.segments[a].comparisons["Best Segments"])
  local b_diff = time.accessor_sub(run.segments[b].comparisons[snapshot.current_comparison], run.segments[b].comparisons["Best Segments"])
  return time.accessor_sub(a_diff, b_diff)
end

return {
  ["name"] = "Possible Time Save",
  ["author"] = "yyna",
  ["settings"] =
  function()
    return settings_factory()
      :plugin(labeled.plugin("Possible Time Save"))
      :color("Value Text: Color", 1, 1, 1, 1)
      :number_range("Value Text: Decimals", 0, 3, 1, 1)
      :header("Information")
      :boolean("Show Total Possible Time Save", false)
  end,
  ["widget"] =
    function()
      local c = {}
      for i = 1,children.len do
        table.insert(c, children.get(i))
      end
      
      local time_access = nil
      if snapshot.current_split ~= nil then
        local current_split = math.min(snapshot.current_split, #run.segments)
        
        if setting("Show Total Possible Time Save") then
          if current_split == 1 then
            time_access = get(#run.segments)
          else
            time_access = difference(#run.segments, current_split - 1)
          end
        else
          if current_split == 1 then
            time_access = get(1)
          else
            time_access = difference(current_split, current_split - 1)
          end
        end
      end
      
      local time_number = nil
      if time_access ~= nil then
        time_number = time.cta(time_access)
      end

      return labeled.apply(
        widgets.text(time.format(time_number, setting("Value Text: Decimals"))):style(setting("Value Text: Color"))
      )
    end
}
