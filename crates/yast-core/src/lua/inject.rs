use anyhow::Result;
use livesplit_core::Timer;
use mlua::prelude::*;

pub fn inject_values_in_lua(lua: &Lua, timer: &Timer) -> Result<()> {
  let snapshot = timer.snapshot();

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

  let run = timer.run();

  let run_table = lua.create_table()?;

  run_table.set("game_name", run.game_name())?;
  run_table.set("game_icon", run.game_icon().data())?;
  run_table.set("category_name", run.category_name())?;
  run_table.set("attempt_count", run.attempt_count())?;

  let metadata = run.metadata();
  let metadata_table = lua.create_table()?;
  metadata_table.set("run_id", metadata.run_id())?;
  metadata_table.set("platform_name", metadata.platform_name())?;
  metadata_table.set("uses_emulator", metadata.uses_emulator())?;
  metadata_table.set("region_name", metadata.region_name())?;
  run_table.set("metadata", metadata_table)?;

  let mut comparison_names: Vec<String> = vec!["Personal Best".to_string()];
  comparison_names.extend(run.custom_comparisons().to_vec());

  let segments_table = lua.create_table()?;
  for (i, segment) in run.segments().iter().enumerate() {
    let segment_table = lua.create_table()?;
    segment_table.set("name", segment.name())?;
    segment_table.set("icon", segment.icon().data())?;

    let comparisons_table = lua.create_table()?;
    for comp_name in &comparison_names {
      let comp_time = segment.comparison(comp_name);
      let comp_table = lua.create_table()?;
      comp_table.set("real_time", comp_time.real_time.map(|t| t.total_seconds()))?;
      comp_table.set("game_time", comp_time.game_time.map(|t| t.total_seconds()))?;
      comparisons_table.set(comp_name.as_str(), comp_table)?;
    }
    segment_table.set("comparisons", comparisons_table)?;

    segments_table.set(i + 1, segment_table)?;
  }
  run_table.set("segments", segments_table)?;

  lua.globals().set("run", run_table)?;

  Ok(())
}
