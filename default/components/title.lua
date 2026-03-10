local sizing = require("sizing")
local background = require("background")
local text = require("text")

return {
	name = "Title",
	author = "yyna",
	settings = function()
		return settings_factory()
			:header("Position & Size")
			:plugin(sizing.plugin)
			:plugin(background.plugin)
			:header("Information")
			:boolean("Show Title", true)
			:boolean("Show Category", true)
			:boolean("Show Attempt Counter", true)
			:boolean("Show Game Icon", true)
			:header("Text")
			:plugin(text.plugin("Title", true, true, false, function(s)
				return s("Show Title")
			end))
			:plugin(text.plugin("Category", true, true, false, function(s)
				return s("Show Category")
			end))
			:plugin(text.plugin("Attempt Counter", true, false, false, function(s)
				return s("Show Attempt Counter")
			end))
	end,
	widget = function()
		local content_column_vec = {}
		if setting("Show Title") then
			table.insert(
				content_column_vec,
				text.write(run.game_name, "Title")
					:style(text.color("Title"))
					:width("fill")
					:height("fill_portion", text.size("Title"))
					:align_x(text.align_x("Title"))
					:align_y("center")
					:into()
			)
		end
		if setting("Show Category") then
			table.insert(
				content_column_vec,
				text.write(run.category_name, "Category")
					:style(text.color("Category"))
					:width("fill")
					:height("fill_portion", text.size("Category"))
					:align_x(text.align_x("Category"))
					:align_y("center")
					:into()
			)
		end

		local content_column =
			widgets.column(content_column_vec):width("fill"):height("fill"):padding(0, 5, 0, 5):into()

		local stack_vec = {}

		local img = run.game_icon
		if img and setting("Show Game Icon") then
			table.insert(
				stack_vec,
				widgets
					.container(widgets.image(img):height("fill"):into())
					:padding(5, 5, 5, 5)
					:width("fill")
					:height("fill")
					:align_x("left")
					:align_y("center")
					:into()
			)
		end

		if setting("Show Attempt Counter") then
			table.insert(
				stack_vec,
				widgets
					.container(
						text.write(tostring(run.attempt_count), "Attempt Counter")
							:style(text.color("Category"))
							:width("fill")
							:height("fill")
							:align_x("right")
							:align_y("bottom")
							:into()
					)
					:padding(5, 5, 5, 5)
					:width("fill")
					:height("fill")
					:align_x("left")
					:align_y("center")
					:into()
			)
		end

		table.insert(stack_vec, content_column)

		return sizing.apply(background.apply(widgets.stack(stack_vec):width("fill"):height("fill"):into()))
	end,
}
