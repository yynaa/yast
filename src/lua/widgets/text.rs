use iced::{
  Color,
  alignment::{Horizontal, Vertical},
  widget::{Text, text},
};
use mlua::prelude::*;

#[derive(Clone)]
pub struct LuaWidgetText {
  content: String,
  align_x: Option<Horizontal>,
  align_y: Option<Vertical>,
  color: Option<Color>,
}

impl LuaWidgetText {
  pub fn new(content: String) -> Self {
    Self {
      content,
      align_x: None,
      align_y: None,
      color: None,
    }
  }

  pub fn build<'a>(self) -> Text<'a> {
    let mut t = text(self.content).color_maybe(self.color);
    if let Some(align_x) = self.align_x {
      t = t.align_x(align_x);
    }
    if let Some(align_y) = self.align_y {
      t = t.align_y(align_y);
    }
    t
  }
}

impl FromLua for LuaWidgetText {
  fn from_lua(value: LuaValue, _: &Lua) -> LuaResult<Self> {
    trace!("{:?}", value);

    match value {
      LuaValue::UserData(ud) => Ok(ud.borrow::<Self>()?.clone()),
      _ => unreachable!(),
    }
  }
}

impl LuaUserData for LuaWidgetText {
  fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
    methods.add_method("align_x", |_, w, s: String| match s.as_str() {
      "left" => Ok(LuaWidgetText {
        align_x: Some(Horizontal::Left),
        ..w.clone()
      }),
      "right" => Ok(LuaWidgetText {
        align_x: Some(Horizontal::Right),
        ..w.clone()
      }),
      "center" => Ok(LuaWidgetText {
        align_x: Some(Horizontal::Center),
        ..w.clone()
      }),
      _ => Err(LuaError::external(anyhow::Error::msg(
        "incorrect alignment",
      ))),
    });

    methods.add_method("align_y", |_, w, s: String| match s.as_str() {
      "bottom" => Ok(LuaWidgetText {
        align_y: Some(Vertical::Bottom),
        ..w.clone()
      }),
      "top" => Ok(LuaWidgetText {
        align_y: Some(Vertical::Top),
        ..w.clone()
      }),
      "center" => Ok(LuaWidgetText {
        align_y: Some(Vertical::Center),
        ..w.clone()
      }),
      _ => Err(LuaError::external(anyhow::Error::msg(
        "incorrect alignment",
      ))),
    });

    methods.add_method("color", |_, w, (r, g, b, a): (f32, f32, f32, f32)| {
      Ok(LuaWidgetText {
        color: Some(Color::from_rgba(r, g, b, a)),
        ..w.clone()
      })
    });
  }
}

pub(super) fn init_lua_widget_text(lua: &Lua, t: &LuaTable) -> LuaResult<()> {
  let constructor = lua.create_function(|_, content: String| Ok(LuaWidgetText::new(content)))?;
  t.set("text", constructor)?;
  Ok(())
}
