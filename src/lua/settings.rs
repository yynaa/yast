use anyhow::Result;
use mlua::prelude::*;

#[derive(Clone)]
pub struct LuaComponentSetting {
  pub name: String,
  pub value: LuaComponentSettingValue,
}

#[derive(Clone)]
pub enum LuaComponentSettingValue {
  Boolean { value: bool, default: bool },
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
    methods.add_method("get", |_, settings, name: String| {
      match settings.values.iter().find(|f| f.name == name) {
        Some(setting) => match setting.value {
          LuaComponentSettingValue::Boolean { value, default: _ } => Ok(value),
        },
        None => Err(LuaError::external(anyhow::Error::msg("missing unit"))),
      }
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
