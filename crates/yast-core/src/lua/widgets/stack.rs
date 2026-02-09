use iced::{Element, Length, widget::stack};
use mlua::prelude::*;

use crate::{layout::component::Component, lua::widgets::LuaWidget};

#[derive(Clone)]
pub struct LuaWidgetStack {
  inner: Vec<LuaWidget>,
  width: Option<Length>,
  height: Option<Length>,
  clip: Option<bool>,
}

impl LuaWidgetStack {
  pub fn new(inner: Vec<LuaWidget>) -> Self {
    Self {
      inner,
      width: None,
      height: None,
      clip: None,
    }
  }

  pub fn build<'a, M: 'a>(self, tree: &Component) -> Element<'a, M> {
    let inner_built = self
      .inner
      .iter()
      .map(|e| e.clone().build(tree))
      .collect::<Vec<Element<'a, M>>>();

    let mut s = stack(inner_built);

    if let Some(width) = self.width {
      s = s.width(width);
    }
    if let Some(height) = self.height {
      s = s.height(height);
    }
    if let Some(clip) = self.clip {
      s = s.clip(clip);
    }

    s.into()
  }
}

impl FromLua for LuaWidgetStack {
  fn from_lua(value: LuaValue, _: &Lua) -> LuaResult<Self> {
    match value {
      LuaValue::UserData(ud) => Ok(ud.borrow::<Self>()?.clone()),
      _ => unreachable!(),
    }
  }
}

impl LuaUserData for LuaWidgetStack {
  fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
    methods.add_method("into", |_, w, ()| Ok(LuaWidget::Stack(w.clone())));

    methods.add_method(
      "width",
      |_, w, (typ, unit): (String, Option<f32>)| match typ.as_str() {
        "fill" => Ok(LuaWidgetStack {
          width: Some(Length::Fill),
          ..w.clone()
        }),
        "fill_portion" => match unit {
          Some(u) => Ok(LuaWidgetStack {
            width: Some(Length::FillPortion(u as u16)),
            ..w.clone()
          }),
          None => Err(LuaError::external(anyhow::Error::msg("missing unit"))),
        },
        "shrink" => Ok(LuaWidgetStack {
          width: Some(Length::Shrink),
          ..w.clone()
        }),
        "fixed" => match unit {
          Some(u) => Ok(LuaWidgetStack {
            width: Some(Length::Fixed(u)),
            ..w.clone()
          }),
          None => Err(LuaError::external(anyhow::Error::msg("missing unit"))),
        },
        _ => Err(LuaError::external(anyhow::Error::msg("incorrect length"))),
      },
    );

    methods.add_method(
      "height",
      |_, w, (typ, unit): (String, Option<f32>)| match typ.as_str() {
        "fill" => Ok(LuaWidgetStack {
          height: Some(Length::Fill),
          ..w.clone()
        }),
        "fill_portion" => match unit {
          Some(u) => Ok(LuaWidgetStack {
            height: Some(Length::FillPortion(u as u16)),
            ..w.clone()
          }),
          None => Err(LuaError::external(anyhow::Error::msg("missing unit"))),
        },
        "shrink" => Ok(LuaWidgetStack {
          height: Some(Length::Shrink),
          ..w.clone()
        }),
        "fixed" => match unit {
          Some(u) => Ok(LuaWidgetStack {
            height: Some(Length::Fixed(u)),
            ..w.clone()
          }),
          None => Err(LuaError::external(anyhow::Error::msg("missing unit"))),
        },
        _ => Err(LuaError::external(anyhow::Error::msg("incorrect length"))),
      },
    );

    methods.add_method("clip", |_, w, clip: bool| {
      Ok(LuaWidgetStack {
        clip: Some(clip),
        ..w.clone()
      })
    });
  }
}

pub(super) fn init_lua_widget_stack(lua: &Lua, t: &LuaTable) -> LuaResult<()> {
  let constructor =
    lua.create_function(|_, inner: Vec<LuaWidget>| Ok(LuaWidgetStack::new(inner)))?;
  t.set("stack", constructor)?;
  Ok(())
}
