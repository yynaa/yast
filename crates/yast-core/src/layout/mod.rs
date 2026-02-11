use anyhow::Result;
use mlua::prelude::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

use crate::layout::{component::Component, settings::LayoutSettings};

pub mod component;
pub mod settings;

#[derive(Serialize, Deserialize)]
pub struct Layout {
  pub name: String,
  pub author: String,
  pub content: Option<Component>,

  pub settings: LayoutSettings,
  pub width: f32,
  pub height: f32,
}

impl Layout {
  pub fn load(components: &HashMap<String, String>, lua: &Lua, content: String) -> Result<Self> {
    let mut layout = toml::from_str::<Self>(&content)?;

    if let Some(root) = &mut layout.content {
      root.load(components, lua)?
    }

    Ok(layout)
  }

  pub fn save(&self, path: &str) -> Result<()> {
    let toml = toml::to_string(self)?;
    fs::write(path, toml)?;
    Ok(())
  }
}

impl Default for Layout {
  fn default() -> Self {
    Self {
      name: String::from("untitled"),
      author: String::new(),
      content: None,

      settings: HashMap::new(),
      width: 200.,
      height: 500.,
    }
  }
}
