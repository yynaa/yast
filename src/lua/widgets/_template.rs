use iced::{
  Color, Element, Length, Pixels,
  alignment::{Horizontal, Vertical},
  widget::text,
};
use mlua::prelude::*;

use crate::{app::AppMessage, lua::widgets::LuaWidget};

#[derive(Clone)]
pub struct LuaWidgetTemplate {}

impl LuaWidgetTemplate {
  pub fn new() -> Self {
    Self {}
  }

  pub fn build<'a>(self) -> Element<'a, AppMessage> {
    todo!()
  }
}

impl FromLua for LuaWidgetTemplate {
  fn from_lua(value: LuaValue, _: &Lua) -> LuaResult<Self> {
    match value {
      LuaValue::UserData(ud) => Ok(ud.borrow::<Self>()?.clone()),
      _ => unreachable!(),
    }
  }
}

impl LuaUserData for LuaWidgetTemplate {
  fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
    methods.add_method("into", |_, w, ()| Ok(LuaWidget::Template(w.clone())));
  }
}

pub(super) fn init_lua_widget_template(lua: &Lua, t: &LuaTable) -> LuaResult<()> {
  let constructor = lua.create_function(|_, ()| Ok(LuaWidgetTemplate::new()))?;
  t.set("template", constructor)?;
  Ok(())
}
