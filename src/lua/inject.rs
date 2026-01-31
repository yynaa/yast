use anyhow::Result;
use mlua::prelude::*;

use crate::app::AppContext;

pub fn inject_values_in_lua(lua: &Lua, context: &AppContext) -> Result<()> {
  let snapshot = context.timer.snapshot();

  let current_attempt_duration = snapshot.current_attempt_duration().total_seconds();
  let current_comparison = snapshot.current_comparison();
  let current_phase = format!("{:?}", snapshot.current_phase());
  let current_split = snapshot.current_split_index();
  let current_time = snapshot.current_time();
  let current_timing_method = format!("{:?}", snapshot.current_timing_method());

  let snapshot_table = lua.create_table()?;

  snapshot_table.set("current_attempt_duration", current_attempt_duration)?;
  snapshot_table.set("current_comparison", current_comparison)?;
  snapshot_table.set("current_phase", current_phase)?;
  snapshot_table.set("current_split", current_split)?;
  snapshot_table.set("current_timing_method", current_timing_method)?;

  let current_time_table = lua.create_table()?;
  current_time_table.set(
    "real_time",
    current_time.real_time.map(|f| f.total_seconds()),
  )?;
  current_time_table.set(
    "game_time",
    current_time.game_time.map(|f| f.total_seconds()),
  )?;
  snapshot_table.set("current_time", current_time_table)?;

  lua.globals().set("snapshot", snapshot_table)?;

  Ok(())
}
