use anyhow::Result;
use mlua::prelude::*;

#[derive(Clone)]
pub struct LuaComponentSetting {
  name: String,
  value: LuaComponentSettingValue,
}

#[derive(Clone)]
pub enum LuaComponentSettingValue {
  Boolean { default: bool },
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

impl LuaUserData for LuaComponentSettings {
  fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
    methods.add_method("boolean", |_, settings, (name, default): (String, bool)| {
      let mut settings = settings.clone();
      settings.values.push(LuaComponentSetting {
        name,
        value: LuaComponentSettingValue::Boolean { default },
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
