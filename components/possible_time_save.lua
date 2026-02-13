local labeled = require("labeled")
local time = require("time")

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
          time_access = time.accessor_or_zero(nil)
          for i = current_split, #run.segments do
            local segment = run.segments[current_split]
            local to_add = time.accessor_sub(segment.comparisons[snapshot.current_comparison], segment.comparisons["Best Segments"])
            time_access = time.accessor_add(time_access, to_add)
          end
        else
          local segment = run.segments[current_split]
          time_access = time.accessor_sub(segment.comparisons[snapshot.current_comparison], segment.comparisons["Best Segments"])
        end
      end
      
      local time_number = nil
      if time_access ~= nil then
        time_number = time.cta(time_access)
      end

      return labeled.apply(
        widgets.text(time.format_delta(time_number, setting("Value Text: Decimals"))):style(setting("Value Text: Color"))
      )
    end
}
