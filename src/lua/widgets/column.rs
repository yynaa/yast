use iced::{
  Color, Element, Length, Pixels,
  alignment::{Horizontal, Vertical},
  widget::{column, text},
};
use mlua::prelude::*;

use crate::{app::AppMessage, layout::LayoutPart, lua::widgets::LuaWidget};

#[derive(Clone)]
pub struct LuaWidgetColumn {
  inner: Vec<LuaWidget>,
}

impl LuaWidgetColumn {
  pub fn new(inner: Vec<LuaWidget>) -> Self {
    Self { inner }
  }

  pub fn build<'a>(self, tree: &Box<dyn LayoutPart>) -> Element<'a, AppMessage> {
    let inner_built = self
      .inner
      .iter()
      .map(|e| e.clone().build(tree))
      .collect::<Vec<Element<'a, AppMessage>>>();

    column(inner_built).into()
  }
}

impl FromLua for LuaWidgetColumn {
  fn from_lua(value: LuaValue, _: &Lua) -> LuaResult<Self> {
    match value {
      LuaValue::UserData(ud) => Ok(ud.borrow::<Self>()?.clone()),
      _ => unreachable!(),
    }
  }
}

impl LuaUserData for LuaWidgetColumn {
  fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
    methods.add_method("into", |_, w, ()| Ok(LuaWidget::Column(w.clone())));
  }
}

pub(super) fn init_lua_widget_column(lua: &Lua, t: &LuaTable) -> LuaResult<()> {
  let constructor =
    lua.create_function(|_, inner: Vec<LuaWidget>| Ok(LuaWidgetColumn::new(inner)))?;
  t.set("column", constructor)?;
  Ok(())
}
