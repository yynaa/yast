use std::fs;

use anyhow::Result;
use include_dir::Dir;
use log::info;

pub fn copy_default_components(default_dir: &Dir<'static>) -> Result<()> {
  let data_dir = dirs::data_dir().expect("couldn't get data directory");
  let mut yast_dir = data_dir.clone();
  yast_dir.push("yast/");

  if !yast_dir.try_exists()? {
    info!("couldn't find defaults in data directory; copying");
    fs::create_dir(yast_dir.clone())?;
    default_dir.extract(yast_dir)?;
  }

  Ok(())
}
