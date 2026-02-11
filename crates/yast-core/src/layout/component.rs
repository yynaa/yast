use std::{
  collections::HashMap,
  fs::{read_dir, read_to_string},
  path::Path,
};

use anyhow::Result;
use iced::Element;
use mlua::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
  layout::settings::LayoutSettings,
  lua::{settings::SettingsFactory, widgets::LuaWidget},
  repository::Repository,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct Component {
  pub name: String,
  pub author: String,
  pub children: Vec<Component>,

  #[serde(skip)]
  widget: Option<LuaFunction>,
  #[serde(skip)]
  pub parameters: SettingsFactory,
}

impl Component {
  pub fn from_str(s: String, lua: &Lua) -> Result<Self> {
    let t = lua.load(s).eval::<LuaTable>()?;

    let r = Self {
      name: t.get("name")?,
      author: t.get("author")?,
      widget: Some(t.get("widget")?),

      parameters: t.call_function("settings", ())?,
      children: Vec::new(),
    };

    Ok(r)
  }

  pub fn load(
    &mut self,
    repository: &mut Repository,
    components: &HashMap<String, String>,
    lua: &Lua,
  ) -> Result<()> {
    let template = Self::from_str(
      components
        .get(&self.name)
        .ok_or(anyhow::Error::msg("missing component"))?
        .clone(),
      lua,
    )?;

    self.widget = template.widget;
    self.parameters = template.parameters;

    for child in &mut self.children {
      child.load(repository, components, lua)?;
    }

    Ok(())
  }

  pub fn get_from_path(&self, path: Vec<usize>) -> Result<&Self> {
    if path.len() == 0 {
      Ok(self)
    } else {
      let s = path.split_at(1);
      self
        .children
        .get(s.0[0])
        .ok_or(anyhow::Error::msg("couldn't find child"))?
        .get_from_path(s.1.to_vec())
    }
  }

  pub fn get_mut_from_path(&mut self, path: Vec<usize>) -> Result<&mut Self> {
    if path.len() == 0 {
      Ok(self)
    } else {
      let s = path.split_at(1);
      self
        .children
        .get_mut(s.0[0])
        .ok_or(anyhow::Error::msg("couldn't find child"))?
        .get_mut_from_path(s.1.to_vec())
    }
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

  pub fn build<'a, M: 'a>(
    &self,
    lua: &Lua,
    path: Vec<usize>,
    layout_settings: &LayoutSettings,
    repository: &Repository,
  ) -> Result<Element<'a, M>> {
    if let Some(widget) = &self.widget {
      let env = widget.environment().ok_or(anyhow::Error::msg(
        "couldn't get environment when building component",
      ))?;

      let pc = path.clone();
      let rc = repository.clone();
      let lsc = layout_settings.clone();
      let lc = lua.clone();
      let setting = lua.create_function(move |_, name: String| {
        let lsc = lsc.clone();

        if let Some(a) = lsc.get(&pc.clone()) {
          if let Some(b) = a.get(&name) {
            Ok(b.inner(&lc, &rc, pc.clone(), name.clone()))
          } else {
            Err(LuaError::external(anyhow::Error::msg(
              "can't find setting in layout settings",
            )))
          }
        } else {
          Err(LuaError::external(anyhow::Error::msg(
            "can't find component in layout settings",
          )))
        }
      })?;
      env.set("setting", setting)?;

      let children = env.get::<LuaTable>("children")?;
      children.set("len", self.children.len())?;
      env.set("children", children)?;

      widget.set_environment(env)?;

      let e = widget
        .call::<LuaWidget>(())?
        .build(&self, lua, path, layout_settings, repository);
      Ok(e)
    } else {
      Err(anyhow::Error::msg(
        "no widget lua function in component (could be a loading error!)",
      ))
    }
  }
}
