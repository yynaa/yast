use anyhow::Result;
use mlua::prelude::*;

use crate::app::AppContext;

pub fn inject_values_in_lua(lua: &Lua, context: &AppContext) -> Result<()> {
  let snapshot = context.timer.snapshot();

  let snapshot_table = lua.create_table()?;

  let current_time_table = lua.create_table()?;
  current_time_table.set(
    "real_time",
    snapshot.current_time().real_time.unwrap().total_seconds(),
  )?;
  snapshot_table.set("current_time", current_time_table)?;

  lua.globals().set("snapshot", snapshot_table)?;

  Ok(())
}
