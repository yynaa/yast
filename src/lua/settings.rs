use anyhow::Result;
use mlua::prelude::*;
use serde::{Deserialize, Serialize};

use crate::lua::widgets::image::ImageHandleLua;

#[derive(Clone, Serialize, Deserialize)]
pub struct LuaComponentSetting {
  pub name: String,
  pub value: LuaComponentSettingValue,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum LuaComponentSettingValue {
  Boolean {
    value: bool,
    #[serde(skip)]
    default: bool,
  },

  String {
    value: String,
    #[serde(skip)]
    default: String,
  },
  Options {
    value: String,
    #[serde(skip)]
    default: String,
    #[serde(skip)]
    options: Vec<String>,
  },

  Number {
    value: f64,
    #[serde(skip)]
    default: f64,
  },
  NumberRange {
    value: f64,
    #[serde(skip)]
    default: f64,
    #[serde(skip)]
    min: f64,
    #[serde(skip)]
    max: f64,
    #[serde(skip)]
    step: f64,
  },

  Color {
    value: [f32; 4],
    #[serde(skip)]
    default: [f32; 4],
  },
  Image {
    bytes: Option<Vec<u8>>,
    #[serde(skip)]
    handle: Option<ImageHandleLua>,
  },
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LuaComponentSettings {
  pub values: Vec<LuaComponentSetting>,
}

impl LuaComponentSettings {
  fn new() -> Self {
    Self { values: Vec::new() }
  }
}

impl FromLua for LuaComponentSettings {
  fn from_lua(value: LuaValue, _: &Lua) -> LuaResult<Self> {
    match value {
      LuaValue::UserData(ud) => Ok(ud.borrow::<Self>()?.clone()),
      _ => unreachable!(),
    }
  }
}

impl LuaUserData for LuaComponentSettings {
  fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
    methods.add_method(
      "get",
      |lua, settings, name: String| -> LuaResult<LuaValue> {
        match settings.values.iter().find(|f| f.name == name) {
          Some(setting) => match &setting.value {
            LuaComponentSettingValue::Boolean { value, default: _ } => {
              Ok(LuaValue::Boolean(*value))
            }
            LuaComponentSettingValue::String { value, default: _ } => {
              Ok(LuaValue::String(lua.create_string(value)?))
            }
            LuaComponentSettingValue::Options {
              value,
              default: _,
              options: _,
            } => Ok(LuaValue::String(lua.create_string(value)?)),
            LuaComponentSettingValue::Number { value, default: _ } => Ok(LuaValue::Number(*value)),
            LuaComponentSettingValue::NumberRange {
              value,
              default: _,
              min: _,
              max: _,
              step: _,
            } => Ok(LuaValue::Number(*value)),
            LuaComponentSettingValue::Color { value, default: _ } => {
              let table = lua.create_table()?;
              table.set(1, value[0])?;
              table.set(2, value[1])?;
              table.set(3, value[2])?;
              table.set(4, value[3])?;
              Ok(LuaValue::Table(table))
            }
            LuaComponentSettingValue::Image { bytes: _, handle } => {
              if let Some(h) = handle {
                Ok(LuaValue::UserData(lua.create_userdata(h.clone())?))
              } else {
                Ok(LuaValue::Nil)
              }
            }
          },
          None => Err(LuaError::external(anyhow::Error::msg("setting not found"))),
        }
      },
    );

    methods.add_method("plugin", |_, settings, t: LuaFunction| {
      let new = t.call::<LuaComponentSettings>(settings.clone())?;
      Ok(new)
    });

    methods.add_method("boolean", |_, settings, (name, default): (String, bool)| {
      let mut settings = settings.clone();
      settings.values.push(LuaComponentSetting {
        name,
        value: LuaComponentSettingValue::Boolean {
          value: default,
          default,
        },
      });
      Ok(settings)
    });

    methods.add_method(
      "string",
      |_, settings, (name, default): (String, String)| {
        let mut settings = settings.clone();
        settings.values.push(LuaComponentSetting {
          name,
          value: LuaComponentSettingValue::String {
            value: default.clone(),
            default,
          },
        });
        Ok(settings)
      },
    );

    methods.add_method(
      "options",
      |_, settings, (name, default, options): (String, String, Vec<String>)| {
        let mut settings = settings.clone();
        settings.values.push(LuaComponentSetting {
          name,
          value: LuaComponentSettingValue::Options {
            value: default.clone(),
            default,
            options,
          },
        });
        Ok(settings)
      },
    );

    methods.add_method("number", |_, settings, (name, default): (String, f64)| {
      let mut settings = settings.clone();
      settings.values.push(LuaComponentSetting {
        name,
        value: LuaComponentSettingValue::Number {
          value: default,
          default,
        },
      });
      Ok(settings)
    });

    methods.add_method(
      "number_range",
      |_, settings, (name, default, min, max, step): (String, f64, f64, f64, f64)| {
        let mut settings = settings.clone();
        settings.values.push(LuaComponentSetting {
          name,
          value: LuaComponentSettingValue::NumberRange {
            value: default,
            default,
            min,
            max,
            step,
          },
        });
        Ok(settings)
      },
    );

    methods.add_method(
      "color",
      |_, settings, (name, r, g, b, a): (String, f32, f32, f32, f32)| {
        let mut settings = settings.clone();
        let value = [r, g, b, a];
        settings.values.push(LuaComponentSetting {
          name,
          value: LuaComponentSettingValue::Color {
            value: value.clone(),
            default: value,
          },
        });
        Ok(settings)
      },
    );

    methods.add_method("image", |_, settings, name: String| {
      let mut settings = settings.clone();
      settings.values.push(LuaComponentSetting {
        name,
        value: LuaComponentSettingValue::Image {
          bytes: None,
          handle: None,
        },
      });
      Ok(settings)
    });
  }
}

pub fn component_settings(lua: &Lua) -> Result<()> {
  let component_settings_constructor =
    lua.create_function(|_, ()| Ok(LuaComponentSettings::new()))?;
  lua
    .globals()
    .set("build_settings", component_settings_constructor)?;
  Ok(())
}
