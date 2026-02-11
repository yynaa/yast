use std::collections::HashMap;

use anyhow::Result;
use iced::advanced::image;
use livesplit_core::Run;

#[derive(Clone, Default)]
pub struct Repository {
  pub layout_images: HashMap<(Vec<usize>, String), Option<image::Handle>>,
  pub game_icon: Option<image::Handle>,
  pub splits_icon: Vec<Option<image::Handle>>,
}

impl Repository {
  pub fn update_from_splits(&mut self, run: &Run) -> Result<()> {
    self.game_icon = if run.game_icon().is_empty() {
      None
    } else {
      let bytes = run.game_icon().data().to_vec();
      Some(image::Handle::from_bytes(bytes))
    };

    self.splits_icon = Vec::new();
    for segment in run.segments() {
      self.splits_icon.push(if segment.icon().is_empty() {
        None
      } else {
        let bytes = segment.icon().data().to_vec();
        Some(image::Handle::from_bytes(bytes))
      });
    }

    Ok(())
  }
}
