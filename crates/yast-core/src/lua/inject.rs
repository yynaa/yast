use anyhow::Result;
use livesplit_core::{Timer, analysis};
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

  let analysis_table = lua.create_table()?;

  let analysis_comparisons_table = lua.create_table()?;
  for comp_name in &comparison_names {
    let comp_table = lua.create_table()?;

    let current_pace = analysis::current_pace::calculate(&snapshot, comp_name);
    let current_pace_table = lua.create_table()?;
    current_pace_table.set("time", current_pace.0.map(|f| f.total_seconds()))?;
    current_pace_table.set("is_live", current_pace.1)?;
    comp_table.set("current_pace", current_pace_table)?;

    let delta = analysis::delta::calculate(&snapshot, comp_name);
    let delta_table = lua.create_table()?;
    delta_table.set("delta", delta.0.map(|f| f.total_seconds()))?;
    delta_table.set("is_live", delta.1)?;
    comp_table.set("delta", delta_table)?;

    let live_delta_real_time = analysis::state_helper::check_live_delta(
      &snapshot,
      false,
      comp_name,
      livesplit_core::TimingMethod::RealTime,
    )
    .map(|f| f.total_seconds());
    let live_delta_game_time = analysis::state_helper::check_live_delta(
      &snapshot,
      false,
      comp_name,
      livesplit_core::TimingMethod::GameTime,
    )
    .map(|f| f.total_seconds());
    let live_delta_table = lua.create_table()?;
    live_delta_table.set("real_time", live_delta_real_time)?;
    live_delta_table.set("game_time", live_delta_game_time)?;
    analysis_table.set("live_delta", live_delta_table)?;

    let live_split_delta_real_time = analysis::state_helper::check_live_delta(
      &snapshot,
      true,
      comp_name,
      livesplit_core::TimingMethod::RealTime,
    )
    .map(|f| f.total_seconds());
    let live_split_delta_game_time = analysis::state_helper::check_live_delta(
      &snapshot,
      true,
      comp_name,
      livesplit_core::TimingMethod::GameTime,
    )
    .map(|f| f.total_seconds());
    let live_split_delta_table = lua.create_table()?;
    live_split_delta_table.set("real_time", live_split_delta_real_time)?;
    live_split_delta_table.set("game_time", live_split_delta_game_time)?;
    analysis_table.set("live_split_delta", live_split_delta_table)?;

    let segments_table = lua.create_table()?;
    for (i, _segment) in run.segments().iter().enumerate() {
      let segment_table = lua.create_table()?;

      let pst = analysis::possible_time_save::calculate(&snapshot, i, comp_name, true);
      let pst_table = lua.create_table()?;
      pst_table.set("time", pst.0.map(|f| f.total_seconds()))?;
      pst_table.set("is_live", pst.1)?;
      segment_table.set("possible_save_time", pst_table)?;

      let total_pst = analysis::possible_time_save::calculate_total(&snapshot, i, comp_name);
      let total_pst_table = lua.create_table()?;
      total_pst_table.set("time", total_pst.0.total_seconds())?;
      total_pst_table.set("is_live", total_pst.1)?;
      segment_table.set("total_possible_save_time", total_pst_table)?;

      let is_best_segment_real_time = analysis::state_helper::check_best_segment(
        &timer,
        i,
        livesplit_core::TimingMethod::RealTime,
      );
      let is_best_segment_game_time = analysis::state_helper::check_best_segment(
        &timer,
        i,
        livesplit_core::TimingMethod::GameTime,
      );
      let is_best_segment_table = lua.create_table()?;
      is_best_segment_table.set("real_time", is_best_segment_real_time)?;
      is_best_segment_table.set("game_time", is_best_segment_game_time)?;
      segment_table.set("is_best_segment", is_best_segment_table)?;

      let last_delta_real_time = analysis::state_helper::last_delta(
        timer.run(),
        i,
        comp_name,
        livesplit_core::TimingMethod::RealTime,
      )
      .map(|f| f.total_seconds());
      let last_delta_game_time = analysis::state_helper::last_delta(
        timer.run(),
        i,
        comp_name,
        livesplit_core::TimingMethod::GameTime,
      )
      .map(|f| f.total_seconds());
      let last_delta_table = lua.create_table()?;
      last_delta_table.set("real_time", last_delta_real_time)?;
      last_delta_table.set("game_time", last_delta_game_time)?;
      segment_table.set("last_delta", last_delta_table)?;

      let live_segment_delta_real_time = analysis::state_helper::live_segment_delta(
        &snapshot,
        i,
        comp_name,
        livesplit_core::TimingMethod::RealTime,
      )
      .map(|f| f.total_seconds());
      let live_segment_delta_game_time = analysis::state_helper::live_segment_delta(
        &snapshot,
        i,
        comp_name,
        livesplit_core::TimingMethod::GameTime,
      )
      .map(|f| f.total_seconds());
      let live_segment_delta_table = lua.create_table()?;
      live_segment_delta_table.set("real_time", live_segment_delta_real_time)?;
      live_segment_delta_table.set("game_time", live_segment_delta_game_time)?;
      segment_table.set("live_segment_delta", live_segment_delta_table)?;

      let previous_segment_delta_real_time = analysis::state_helper::previous_segment_delta(
        &snapshot,
        i,
        comp_name,
        livesplit_core::TimingMethod::RealTime,
      )
      .map(|f| f.total_seconds());
      let previous_segment_delta_game_time = analysis::state_helper::previous_segment_delta(
        &snapshot,
        i,
        comp_name,
        livesplit_core::TimingMethod::GameTime,
      )
      .map(|f| f.total_seconds());
      let previous_segment_delta_table = lua.create_table()?;
      previous_segment_delta_table.set("real_time", previous_segment_delta_real_time)?;
      previous_segment_delta_table.set("game_time", previous_segment_delta_game_time)?;
      segment_table.set("previous_segment_delta", previous_segment_delta_table)?;

      segments_table.set(i, segment_table)?;
    }
    comp_table.set("segments", segments_table)?;

    analysis_comparisons_table.set(comp_name.as_str(), comp_table)?;
  }
  analysis_table.set("comparisons", analysis_comparisons_table)?;

  let segments_table = lua.create_table()?;
  for (i, _segment) in run.segments().iter().enumerate() {
    let segment_table = lua.create_table()?;

    let live_segment_time_real_time = analysis::state_helper::live_segment_time(
      &snapshot,
      i,
      livesplit_core::TimingMethod::RealTime,
    )
    .map(|f| f.total_seconds());
    let live_segment_time_game_time = analysis::state_helper::live_segment_time(
      &snapshot,
      i,
      livesplit_core::TimingMethod::GameTime,
    )
    .map(|f| f.total_seconds());
    let live_segment_time_table = lua.create_table()?;
    live_segment_time_table.set("real_time", live_segment_time_real_time)?;
    live_segment_time_table.set("game_time", live_segment_time_game_time)?;
    segment_table.set("live_segment_time", live_segment_time_table)?;

    let previous_segment_time_real_time = analysis::state_helper::previous_segment_time(
      &snapshot,
      i,
      livesplit_core::TimingMethod::RealTime,
    )
    .map(|f| f.total_seconds());
    let previous_segment_time_game_time = analysis::state_helper::previous_segment_time(
      &snapshot,
      i,
      livesplit_core::TimingMethod::GameTime,
    )
    .map(|f| f.total_seconds());
    let previous_segment_time_table = lua.create_table()?;
    previous_segment_time_table.set("real_time", previous_segment_time_real_time)?;
    previous_segment_time_table.set("game_time", previous_segment_time_game_time)?;
    segment_table.set("previous_segment_time", previous_segment_time_table)?;

    segments_table.set(i, segment_table)?;
  }
  analysis_table.set("segments", segments_table)?;

  let pb_chance = analysis::pb_chance::for_timer(&snapshot);
  let pb_chance_table = lua.create_table()?;
  pb_chance_table.set("chance", pb_chance.0)?;
  pb_chance_table.set("is_live", pb_chance.1)?;
  analysis_table.set("pb_chance", pb_chance_table)?;

  let sum_of_best_segments_real_time = analysis::sum_of_segments::calculate_best(
    timer.run().segments(),
    false,
    false,
    livesplit_core::TimingMethod::RealTime,
  )
  .map(|f| f.total_seconds());
  let sum_of_best_segments_game_time = analysis::sum_of_segments::calculate_best(
    timer.run().segments(),
    false,
    false,
    livesplit_core::TimingMethod::GameTime,
  )
  .map(|f| f.total_seconds());
  let sum_of_best_segments_table = lua.create_table()?;
  sum_of_best_segments_table.set("real_time", sum_of_best_segments_real_time)?;
  sum_of_best_segments_table.set("game_time", sum_of_best_segments_game_time)?;
  analysis_table.set("sum_of_best_segments", sum_of_best_segments_table)?;

  let sum_of_worst_segments_real_time = analysis::sum_of_segments::calculate_worst(
    timer.run().segments(),
    false,
    livesplit_core::TimingMethod::RealTime,
  )
  .map(|f| f.total_seconds());
  let sum_of_worst_segments_game_time = analysis::sum_of_segments::calculate_worst(
    timer.run().segments(),
    false,
    livesplit_core::TimingMethod::GameTime,
  )
  .map(|f| f.total_seconds());
  let sum_of_worst_segments_table = lua.create_table()?;
  sum_of_worst_segments_table.set("real_time", sum_of_worst_segments_real_time)?;
  sum_of_worst_segments_table.set("game_time", sum_of_worst_segments_game_time)?;
  analysis_table.set("sum_of_worst_segments", sum_of_worst_segments_table)?;

  let total_playtime = analysis::total_playtime::calculate(&timer).total_seconds();
  analysis_table.set("total_playtime", total_playtime)?;

  lua.globals().set("analysis", analysis_table)?;

  Ok(())
}
