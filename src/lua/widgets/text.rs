use iced::{
  alignment::{Horizontal, Vertical},
  widget::text,
  Color, Element, Length, Pixels,
};
use mlua::prelude::*;

use crate::{app::AppMessage, lua::widgets::LuaWidget};

#[derive(Clone)]
pub struct LuaWidgetText {
  content: String,
  align_x: Option<Horizontal>,
  align_y: Option<Vertical>,
  style: Option<text::Style>,
  width: Option<Length>,
  height: Option<Length>,
  size: Option<Pixels>,
}

impl LuaWidgetText {
  pub fn new(content: String) -> Self {
    Self {
      content,
      align_x: None,
      align_y: None,
      style: None,
      width: None,
      height: None,
      size: None,
    }
  }

  pub fn build<'a>(self) -> Element<'a, AppMessage> {
    let mut t = text(self.content);
    if let Some(align_x) = self.align_x {
      t = t.align_x(align_x);
    }
    if let Some(align_y) = self.align_y {
      t = t.align_y(align_y);
    }
    if let Some(style) = self.style {
      t = t.style(move |_| style);
    }
    if let Some(width) = self.width {
      t = t.width(width);
    }
    if let Some(height) = self.height {
      t = t.height(height);
    }
    if let Some(size) = self.size {
      t = t.size(size);
    }
    t.into()
  }
}

impl FromLua for LuaWidgetText {
  fn from_lua(value: LuaValue, _: &Lua) -> LuaResult<Self> {
    match value {
      LuaValue::UserData(ud) => Ok(ud.borrow::<Self>()?.clone()),
      _ => unreachable!(),
    }
  }
}

impl LuaUserData for LuaWidgetText {
  fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
    methods.add_method("into", |_, w, ()| Ok(LuaWidget::Text(w.clone())));

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

    methods.add_method("style", |_, w, color: Option<[f32; 4]>| {
      Ok(LuaWidgetText {
        style: Some(text::Style {
          color: color.map(|c| Color::from_rgba(c[0], c[1], c[2], c[3])),
        }),
        ..w.clone()
      })
    });

    methods.add_method(
      "width",
      |_, w, (typ, unit): (String, Option<f32>)| match typ.as_str() {
        "fill" => Ok(LuaWidgetText {
          width: Some(Length::Fill),
          ..w.clone()
        }),
        "fill_portion" => match unit {
          Some(u) => Ok(LuaWidgetText {
            width: Some(Length::FillPortion(u as u16)),
            ..w.clone()
          }),
          None => Err(LuaError::external(anyhow::Error::msg("missing unit"))),
        },
        "shrink" => Ok(LuaWidgetText {
          width: Some(Length::Shrink),
          ..w.clone()
        }),
        "fixed" => match unit {
          Some(u) => Ok(LuaWidgetText {
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
        "fill" => Ok(LuaWidgetText {
          height: Some(Length::Fill),
          ..w.clone()
        }),
        "fill_portion" => match unit {
          Some(u) => Ok(LuaWidgetText {
            height: Some(Length::FillPortion(u as u16)),
            ..w.clone()
          }),
          None => Err(LuaError::external(anyhow::Error::msg("missing unit"))),
        },
        "shrink" => Ok(LuaWidgetText {
          height: Some(Length::Shrink),
          ..w.clone()
        }),
        "fixed" => match unit {
          Some(u) => Ok(LuaWidgetText {
            height: Some(Length::Fixed(u)),
            ..w.clone()
          }),
          None => Err(LuaError::external(anyhow::Error::msg("missing unit"))),
        },
        _ => Err(LuaError::external(anyhow::Error::msg("incorrect length"))),
      },
    );

    methods.add_method("size", |_, w, size: f32| {
      Ok(LuaWidgetText {
        size: Some(Pixels(size)),
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
