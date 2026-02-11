use std::collections::HashMap;

use iced::advanced::image;

#[derive(Clone, Default)]
pub struct Repository {
  pub layout_images: HashMap<(Vec<usize>, String), Option<image::Handle>>,
}
