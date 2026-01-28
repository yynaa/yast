use anyhow::Result;

use crate::app::run_app;

mod app;

fn main() -> Result<()> {
  run_app()?;

  Ok(())
}
