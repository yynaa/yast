use anyhow::Result;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use crate::app::run_app;

pub mod app;
pub mod lua;

fn main() -> Result<()> {
  pretty_env_logger::init_timed();

  run_app()?;

  Ok(())
}
