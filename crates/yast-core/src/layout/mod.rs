use anyhow::Result;
use handy_keys::Hotkey;
use iced::advanced::image;
use mlua::prelude::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};
use strum::EnumIter;

use crate::{
  layout::{
    component::Component,
    settings::{LayoutSettings, SettingsValue},
  },
  repository::Repository,
};

pub mod component;
pub mod moving;
pub mod settings;

#[derive(Serialize, Deserialize)]
pub struct Layout {
  pub name: String,
  pub author: String,
  pub content: Option<Component>,

  pub settings: LayoutSettings,
  pub hotkeys: HashMap<HotkeyAction, Hotkey>,
  pub width: f32,
  pub height: f32,
}

impl Layout {
  pub fn load(
    repository: &mut Repository,
    components: &HashMap<String, String>,
    lua: &Lua,
    content: String,
  ) -> Result<Self> {
    let mut layout = ron::from_str::<Self>(&content)?;

    for (comp_path, comp_parameters) in &layout.settings {
      for (param_name, param_value) in comp_parameters {
        match param_value {
          SettingsValue::Image(b) => {
            repository.layout_images.insert(
              (comp_path.clone(), param_name.clone()),
              b.clone().map(|bb| image::Handle::from_bytes(bb)),
            );
          }
          _ => {}
        }
      }
    }

    if let Some(root) = &mut layout.content {
      root.load(vec![], &mut layout.settings, repository, components, lua)?
    }

    Ok(layout)
  }

  pub fn save(&self, path: &str) -> Result<()> {
    let s = ron::to_string(self)?;
    fs::write(path, s)?;
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
      hotkeys: HashMap::new(),
      width: 200.,
      height: 500.,
    }
  }
}

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, Debug, Clone, EnumIter)]
pub enum HotkeyAction {
  StartOrSplitTimer,
  ResetTimer,
  PauseTimer,
}
