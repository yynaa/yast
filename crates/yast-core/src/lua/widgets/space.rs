use iced::{Element, Length};
use mlua::prelude::*;

use crate::lua::widgets::LuaWidget;

#[derive(Clone)]
pub struct LuaWidgetSpace {
  width: Option<Length>,
  height: Option<Length>,
}

impl LuaWidgetSpace {
  pub fn new() -> Self {
    Self {
      width: None,
      height: None,
    }
  }

  pub fn build<'a, M: 'a>(self) -> Element<'a, M> {
    let mut s = iced::widget::space();
    if let Some(width) = self.width {
      s = s.width(width);
    }
    if let Some(height) = self.height {
      s = s.height(height);
    }
    s.into()
  }
}

impl FromLua for LuaWidgetSpace {
  fn from_lua(value: LuaValue, _: &Lua) -> LuaResult<Self> {
    match value {
      LuaValue::UserData(ud) => Ok(ud.borrow::<Self>()?.clone()),
      _ => unreachable!(),
    }
  }
}

impl LuaUserData for LuaWidgetSpace {
  fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
    methods.add_method("into", |_, w, ()| Ok(LuaWidget::Space(w.clone())));

    methods.add_method(
      "width",
      |_, w, (typ, unit): (String, Option<f32>)| match typ.as_str() {
        "fill" => Ok(LuaWidgetSpace {
          width: Some(Length::Fill),
          ..w.clone()
        }),
        "fill_portion" => match unit {
          Some(u) => Ok(LuaWidgetSpace {
            width: Some(Length::FillPortion(u as u16)),
            ..w.clone()
          }),
          None => Err(LuaError::external(anyhow::Error::msg("missing unit"))),
        },
        "shrink" => Ok(LuaWidgetSpace {
          width: Some(Length::Shrink),
          ..w.clone()
        }),
        "fixed" => match unit {
          Some(u) => Ok(LuaWidgetSpace {
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
        "fill" => Ok(LuaWidgetSpace {
          height: Some(Length::Fill),
          ..w.clone()
        }),
        "fill_portion" => match unit {
          Some(u) => Ok(LuaWidgetSpace {
            height: Some(Length::FillPortion(u as u16)),
            ..w.clone()
          }),
          None => Err(LuaError::external(anyhow::Error::msg("missing unit"))),
        },
        "shrink" => Ok(LuaWidgetSpace {
          height: Some(Length::Shrink),
          ..w.clone()
        }),
        "fixed" => match unit {
          Some(u) => Ok(LuaWidgetSpace {
            height: Some(Length::Fixed(u)),
            ..w.clone()
          }),
          None => Err(LuaError::external(anyhow::Error::msg("missing unit"))),
        },
        _ => Err(LuaError::external(anyhow::Error::msg("incorrect length"))),
      },
    );
  }
}

pub(super) fn init_lua_widget_space(lua: &Lua, t: &LuaTable) -> LuaResult<()> {
  let constructor = lua.create_function(|_, ()| Ok(LuaWidgetSpace::new()))?;
  t.set("space", constructor)?;
  Ok(())
}
