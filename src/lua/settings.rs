use anyhow::Result;
use iced::widget::image;
use mlua::prelude::*;

use crate::lua::widgets::image::ImageHandleLua;

#[derive(Clone)]
pub struct LuaComponentSetting {
  pub name: String,
  pub value: LuaComponentSettingValue,
}

#[derive(Clone)]
pub enum LuaComponentSettingValue {
  Boolean {
    value: bool,
    default: bool,
  },

  String {
    value: String,
    default: String,
  },
  Options {
    value: String,
    default: String,
    options: Vec<String>,
  },

  Number {
    value: f64,
    default: f64,
  },
  NumberRange {
    value: f64,
    default: f64,
    min: f64,
    max: f64,
    step: f64,
  },

  Color {
    value: [f32; 4],
    default: [f32; 4],
  },
  Image {
    bytes: Option<Vec<u8>>,
    handle: Option<ImageHandleLua>,
  },
}

#[derive(Clone)]
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
