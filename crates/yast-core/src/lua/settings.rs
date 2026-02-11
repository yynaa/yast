use std::collections::HashMap;

use anyhow::Result;
use mlua::prelude::*;

use crate::layout::settings::SettingsValue;

#[derive(Clone)]
pub struct SettingsFactoryEntry {
  pub content: SettingsFactoryEntryContent,
  pub show_if: Option<LuaFunction>,
}

#[derive(Clone)]
pub enum SettingsFactoryEntryContent {
  Header(String),
  Value(String, SettingsFactoryValue),
}

#[derive(Clone)]
pub enum SettingsFactoryValue {
  Boolean(bool),
  String(String),
  Options(Vec<String>, String),
  Number(f64),
  NumberRange(f64, f64, f64, f64),
  Color([f32; 4]),
  Image,
}

impl SettingsFactoryValue {
  fn to_settings_value(&self) -> SettingsValue {
    match self {
      Self::Boolean(d) => SettingsValue::Boolean(*d),
      Self::String(d) => SettingsValue::String(d.clone()),
      Self::Options(_, d) => SettingsValue::Options(d.clone()),
      Self::Number(d) => SettingsValue::Number(*d),
      Self::NumberRange(_, _, _, d) => SettingsValue::NumberRange(*d),
      Self::Color(d) => SettingsValue::Color(d.clone()),
      Self::Image => SettingsValue::Image(None),
    }
  }
}

#[derive(Clone)]
pub struct SettingsFactory(pub Vec<SettingsFactoryEntry>);

impl Default for SettingsFactory {
  fn default() -> Self {
    Self(Vec::new())
  }
}

impl FromLua for SettingsFactory {
  fn from_lua(value: LuaValue, _: &Lua) -> LuaResult<Self> {
    match value {
      LuaValue::UserData(ud) => Ok(ud.borrow::<Self>()?.clone()),
      _ => unreachable!(),
    }
  }
}

impl LuaUserData for SettingsFactory {
  fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
    methods.add_method("plugin", |_, settings, t: LuaFunction| {
      let new = t.call::<SettingsFactory>(settings.clone())?;
      Ok(new)
    });

    methods.add_method(
      "header",
      |_, settings, (t, show_if): (String, Option<LuaFunction>)| {
        let mut settings = settings.clone();
        settings.0.push(SettingsFactoryEntry {
          content: SettingsFactoryEntryContent::Header(t),
          show_if,
        });
        Ok(settings)
      },
    );

    methods.add_method(
      "boolean",
      |_, settings, (name, default, show_if): (String, bool, Option<LuaFunction>)| {
        let mut settings = settings.clone();
        settings.0.push(SettingsFactoryEntry {
          content: SettingsFactoryEntryContent::Value(name, SettingsFactoryValue::Boolean(default)),
          show_if,
        });
        Ok(settings)
      },
    );

    methods.add_method(
      "string",
      |_, settings, (name, default, show_if): (String, String, Option<LuaFunction>)| {
        let mut settings = settings.clone();
        settings.0.push(SettingsFactoryEntry {
          content: SettingsFactoryEntryContent::Value(name, SettingsFactoryValue::String(default)),
          show_if,
        });
        Ok(settings)
      },
    );

    methods.add_method(
      "options",
      |_, settings, (name, options, default, show_if): (String, Vec<String>, String, Option<LuaFunction>)| {
        let mut settings = settings.clone();
        settings.0.push(SettingsFactoryEntry {
          content: SettingsFactoryEntryContent::Value(name, SettingsFactoryValue::Options(options, default)),
          show_if,
        });
        Ok(settings)
      },
    );

    methods.add_method(
      "number",
      |_, settings, (name, default, show_if): (String, f64, Option<LuaFunction>)| {
        let mut settings = settings.clone();
        settings.0.push(SettingsFactoryEntry {
          content: SettingsFactoryEntryContent::Value(name, SettingsFactoryValue::Number(default)),
          show_if,
        });
        Ok(settings)
      },
    );

    methods.add_method(
      "number_range",
      |_,
       settings,
       (name, min, max, step, default, show_if): (
        String,
        f64,
        f64,
        f64,
        f64,
        Option<LuaFunction>,
      )| {
        let mut settings = settings.clone();
        settings.0.push(SettingsFactoryEntry {
          content: SettingsFactoryEntryContent::Value(
            name,
            SettingsFactoryValue::NumberRange(min, max, step, default),
          ),
          show_if,
        });
        Ok(settings)
      },
    );

    methods.add_method(
      "color",
      |_,
       settings,
       (name, r, g, b, a, show_if): (String, f32, f32, f32, f32, Option<LuaFunction>)| {
        let mut settings = settings.clone();
        settings.0.push(SettingsFactoryEntry {
          content: SettingsFactoryEntryContent::Value(
            name,
            SettingsFactoryValue::Color([r, g, b, a]),
          ),
          show_if,
        });
        Ok(settings)
      },
    );

    methods.add_method(
      "image",
      |_, settings, (name, show_if): (String, Option<LuaFunction>)| {
        let mut settings = settings.clone();
        settings.0.push(SettingsFactoryEntry {
          content: SettingsFactoryEntryContent::Value(name, SettingsFactoryValue::Image),
          show_if,
        });
        Ok(settings)
      },
    );
  }
}

impl SettingsFactory {
  pub fn initialize_defaults(&self) -> HashMap<String, SettingsValue> {
    let mut r = HashMap::new();
    for entry in &self.0 {
      if let SettingsFactoryEntryContent::Value(name, value) = &entry.content {
        r.insert(name.clone(), value.to_settings_value());
      }
    }
    r
  }
}

pub fn component_settings(lua: &Lua) -> Result<()> {
  let component_settings_constructor =
    lua.create_function(|_, ()| Ok(SettingsFactory::default()))?;
  lua
    .globals()
    .set("settings_factory", component_settings_constructor)?;
  Ok(())
}
