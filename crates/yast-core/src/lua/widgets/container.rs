use iced::{
  Color, Element, Length, Padding,
  alignment::{Horizontal, Vertical},
  widget::container,
};
use mlua::prelude::*;

use crate::{
  layout::{component::Component, settings::LayoutSettings},
  lua::widgets::LuaWidget,
};

#[derive(Clone)]
pub struct LuaWidgetContainer {
  inner: Box<LuaWidget>,
  style: Option<container::Style>,
  padding: Option<Padding>,
  width: Option<Length>,
  height: Option<Length>,
  align_x: Option<Horizontal>,
  align_y: Option<Vertical>,
  clip: Option<bool>,
}

impl LuaWidgetContainer {
  pub fn new(inner: LuaWidget) -> Self {
    Self {
      inner: Box::new(inner),
      style: None,
      padding: None,
      width: None,
      height: None,
      align_x: None,
      align_y: None,
      clip: None,
    }
  }

  pub fn build<'a, M: 'a>(
    self,
    tree: &Component,
    lua: &Lua,
    path: Vec<usize>,
    layout_settings: &LayoutSettings,
  ) -> Element<'a, M> {
    let inner_built = self.inner.build(tree, lua, path.clone(), layout_settings);

    let mut c = container(inner_built);

    if let Some(style) = self.style {
      c = c.style(move |_| style);
    }
    if let Some(padding) = self.padding {
      c = c.padding(padding);
    }
    if let Some(width) = self.width {
      c = c.width(width);
    }
    if let Some(height) = self.height {
      c = c.height(height);
    }
    if let Some(align_x) = self.align_x {
      c = c.align_x(align_x);
    }
    if let Some(align_y) = self.align_y {
      c = c.align_y(align_y);
    }
    if let Some(clip) = self.clip {
      c = c.clip(clip);
    }

    c.into()
  }
}

impl FromLua for LuaWidgetContainer {
  fn from_lua(value: LuaValue, _: &Lua) -> LuaResult<Self> {
    match value {
      LuaValue::UserData(ud) => Ok(ud.borrow::<Self>()?.clone()),
      _ => unreachable!(),
    }
  }
}

impl LuaUserData for LuaWidgetContainer {
  fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
    methods.add_method("into", |_, w, ()| Ok(LuaWidget::Container(w.clone())));

    methods.add_method("padding", |_, w, (t, r, b, l): (f32, f32, f32, f32)| {
      Ok(LuaWidgetContainer {
        padding: Some(Padding {
          top: t,
          right: r,
          bottom: b,
          left: l,
        }),
        ..w.clone()
      })
    });

    methods.add_method(
      "width",
      |_, w, (typ, unit): (String, Option<f32>)| match typ.as_str() {
        "fill" => Ok(LuaWidgetContainer {
          width: Some(Length::Fill),
          ..w.clone()
        }),
        "fill_portion" => match unit {
          Some(u) => Ok(LuaWidgetContainer {
            width: Some(Length::FillPortion(u as u16)),
            ..w.clone()
          }),
          None => Err(LuaError::external(anyhow::Error::msg("missing unit"))),
        },
        "shrink" => Ok(LuaWidgetContainer {
          width: Some(Length::Shrink),
          ..w.clone()
        }),
        "fixed" => match unit {
          Some(u) => Ok(LuaWidgetContainer {
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
        "fill" => Ok(LuaWidgetContainer {
          height: Some(Length::Fill),
          ..w.clone()
        }),
        "fill_portion" => match unit {
          Some(u) => Ok(LuaWidgetContainer {
            height: Some(Length::FillPortion(u as u16)),
            ..w.clone()
          }),
          None => Err(LuaError::external(anyhow::Error::msg("missing unit"))),
        },
        "shrink" => Ok(LuaWidgetContainer {
          height: Some(Length::Shrink),
          ..w.clone()
        }),
        "fixed" => match unit {
          Some(u) => Ok(LuaWidgetContainer {
            height: Some(Length::Fixed(u)),
            ..w.clone()
          }),
          None => Err(LuaError::external(anyhow::Error::msg("missing unit"))),
        },
        _ => Err(LuaError::external(anyhow::Error::msg("incorrect length"))),
      },
    );

    methods.add_method("align_x", |_, w, s: String| match s.as_str() {
      "left" => Ok(LuaWidgetContainer {
        align_x: Some(Horizontal::Left),
        ..w.clone()
      }),
      "right" => Ok(LuaWidgetContainer {
        align_x: Some(Horizontal::Right),
        ..w.clone()
      }),
      "center" => Ok(LuaWidgetContainer {
        align_x: Some(Horizontal::Center),
        ..w.clone()
      }),
      _ => Err(LuaError::external(anyhow::Error::msg(
        "incorrect alignment",
      ))),
    });

    methods.add_method(
      "style",
      |_, w, (text_color, background_color): (Option<[f32; 4]>, Option<[f32; 4]>)| {
        Ok(LuaWidgetContainer {
          style: Some(container::Style {
            text_color: Some(
              Color::from_rgba(
                text_color.map(|c| c[0]).unwrap_or(0.0),
                text_color.map(|c| c[1]).unwrap_or(0.0),
                text_color.map(|c| c[2]).unwrap_or(0.0),
                text_color.map(|c| c[3]).unwrap_or(0.0),
              )
              .into(),
            ),
            background: Some(
              Color::from_rgba(
                background_color.map(|c| c[0]).unwrap_or(0.0),
                background_color.map(|c| c[1]).unwrap_or(0.0),
                background_color.map(|c| c[2]).unwrap_or(0.0),
                background_color.map(|c| c[3]).unwrap_or(0.0),
              )
              .into(),
            ),
            border: iced::Border {
              color: Color::TRANSPARENT,
              width: 0.0,
              radius: 0.0.into(),
            },
            shadow: iced::Shadow {
              color: Color::TRANSPARENT,
              offset: iced::Vector { x: 0.0, y: 0.0 },
              blur_radius: 0.0,
            },
            snap: false,
          }),
          ..w.clone()
        })
      },
    );

    methods.add_method("align_y", |_, w, s: String| match s.as_str() {
      "top" => Ok(LuaWidgetContainer {
        align_y: Some(Vertical::Top),
        ..w.clone()
      }),
      "bottom" => Ok(LuaWidgetContainer {
        align_y: Some(Vertical::Bottom),
        ..w.clone()
      }),
      "center" => Ok(LuaWidgetContainer {
        align_y: Some(Vertical::Center),
        ..w.clone()
      }),
      _ => Err(LuaError::external(anyhow::Error::msg(
        "incorrect alignment",
      ))),
    });

    methods.add_method("clip", |_, w, clip: bool| {
      Ok(LuaWidgetContainer {
        clip: Some(clip),
        ..w.clone()
      })
    });
  }
}

pub(super) fn init_lua_widget_container(lua: &Lua, t: &LuaTable) -> LuaResult<()> {
  let constructor =
    lua.create_function(|_, inner: LuaWidget| Ok(LuaWidgetContainer::new(inner)))?;
  t.set("container", constructor)?;
  Ok(())
}
