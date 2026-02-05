use std::{
  collections::HashMap,
  fs::{read_dir, read_to_string},
  path::Path,
};

use anyhow::Result;
use iced::{Element, widget::image};
use mlua::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
  app::AppMessage,
  lua::{
    settings::{LuaComponentSettingValue, LuaComponentSettings},
    widgets::{LuaWidget, image::ImageHandleLua},
  },
};

#[derive(Clone, Serialize, Deserialize)]
pub struct Component {
  name: String,
  author: String,

  #[serde(skip)]
  widget: Option<LuaFunction>,

  parameters: LuaComponentSettings,
  children: Vec<Component>,
}

impl Component {
  pub fn from_str(s: String, lua: &Lua) -> Result<Self> {
    let t = lua.load(s).eval::<LuaTable>()?;

    let r = Self {
      name: t.get("name")?,
      author: t.get("author")?,
      widget: Some(t.get("widget")?),

      parameters: t.get("settings")?,
      children: Vec::new(),
    };

    Ok(r)
  }

  pub fn load(&mut self, components: &HashMap<String, String>, lua: &Lua) -> Result<()> {
    let template = Self::from_str(
      components
        .get(&self.name)
        .ok_or(anyhow::Error::msg("missing component"))?
        .clone(),
      lua,
    )?;

    self.widget = template.widget;

    for i in 0..self.parameters.values.len() {
      match &self.parameters.values[i].value {
        LuaComponentSettingValue::Boolean { value, default: _ } => {
          if let LuaComponentSettingValue::Boolean { value: _, default } =
            &template.parameters.values[i].value
          {
            self.parameters.values[i].value = LuaComponentSettingValue::Boolean {
              value: value.clone(),
              default: default.clone(),
            };
          }
        }
        LuaComponentSettingValue::String { value, default: _ } => {
          if let LuaComponentSettingValue::String { value: _, default } =
            &template.parameters.values[i].value
          {
            self.parameters.values[i].value = LuaComponentSettingValue::String {
              value: value.clone(),
              default: default.clone(),
            };
          }
        }
        LuaComponentSettingValue::Options {
          value,
          default: _,
          options: _,
        } => {
          if let LuaComponentSettingValue::Options {
            value: _,
            default,
            options,
          } = &template.parameters.values[i].value
          {
            self.parameters.values[i].value = LuaComponentSettingValue::Options {
              value: value.to_string(),
              default: default.clone(),
              options: options.clone(),
            };
          }
        }
        LuaComponentSettingValue::Number { value, default: _ } => {
          if let LuaComponentSettingValue::Number { value: _, default } =
            &template.parameters.values[i].value
          {
            self.parameters.values[i].value = LuaComponentSettingValue::Number {
              value: value.clone(),
              default: default.clone(),
            };
          }
        }
        LuaComponentSettingValue::NumberRange {
          value,
          default: _,
          min: _,
          max: _,
          step: _,
        } => {
          if let LuaComponentSettingValue::NumberRange {
            value: _,
            default,
            min,
            max,
            step,
          } = &template.parameters.values[i].value
          {
            self.parameters.values[i].value = LuaComponentSettingValue::NumberRange {
              value: value.clone(),
              default: default.clone(),
              min: min.clone(),
              max: max.clone(),
              step: step.clone(),
            };
          }
        }
        LuaComponentSettingValue::Color { value, default: _ } => {
          if let LuaComponentSettingValue::Color { value: _, default } =
            &template.parameters.values[i].value
          {
            self.parameters.values[i].value = LuaComponentSettingValue::Color {
              value: value.clone(),
              default: default.clone(),
            };
          }
        }
        LuaComponentSettingValue::Image { bytes, handle: _ } => {
          if let LuaComponentSettingValue::Image {
            bytes: _,
            handle: _,
          } = &template.parameters.values[i].value
          {
            let handle = bytes
              .clone()
              .map(|b| ImageHandleLua(image::Handle::from_bytes(b)));

            self.parameters.values[i].value = LuaComponentSettingValue::Image {
              bytes: bytes.clone(),
              handle,
            };
          }
        }
        _ => todo!(),
      }
    }

    for child in &mut self.children {
      child.load(components, lua)?;
    }

    Ok(())
  }

  pub fn import_all_from_directory(p: &str, lua: &Lua) -> Result<HashMap<String, String>> {
    let path = Path::new(p);
    let mut components = HashMap::new();
    if path.is_dir() {
      for file in read_dir(path)? {
        let entry = file?.path();
        if entry.is_file() {
          let st = read_to_string(entry)?;
          let name = lua
            .load(st.clone())
            .eval::<LuaTable>()?
            .get::<String>("name")?;
          components.insert(name, st);
        }
      }
    }
    Ok(components)
  }

  pub fn build<'a>(&self) -> Result<Element<'a, AppMessage>> {
    if let Some(widget) = &self.widget {
      let env = widget.environment().unwrap();

      env.set("settings", self.parameters.clone())?;

      let children = env.get::<LuaTable>("children")?;
      children.set("len", self.children.len())?;
      env.set("children", children)?;

      widget.set_environment(env)?;

      let e = widget.call::<LuaWidget>(())?.build(&self);
      Ok(e)
    } else {
      Err(anyhow::Error::msg(
        "no widget lua function in component (could be a loading error!)",
      ))
    }
  }

  pub fn get_name(&self) -> String {
    self.name.clone()
  }

  pub fn get_author(&self) -> String {
    self.author.clone()
  }

  pub fn get_children(&self) -> Option<&Vec<Component>> {
    Some(&self.children)
  }

  pub fn get_children_mut(&mut self) -> Option<&mut Vec<Component>> {
    Some(&mut self.children)
  }

  pub fn get_parameters(&self) -> &LuaComponentSettings {
    &self.parameters
  }

  pub fn get_parameters_mut(&mut self) -> &mut LuaComponentSettings {
    &mut self.parameters
  }
}
