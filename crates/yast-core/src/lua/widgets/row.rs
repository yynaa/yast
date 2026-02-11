use iced::{Element, Length, Padding, Pixels, alignment::Vertical, widget::row};
use mlua::prelude::*;

use crate::{
  layout::{component::Component, settings::LayoutSettings},
  lua::widgets::LuaWidget,
};

#[derive(Clone)]
pub struct LuaWidgetRow {
  inner: Vec<LuaWidget>,
  spacing: Option<Pixels>,
  padding: Option<Padding>,
  width: Option<Length>,
  height: Option<Length>,
  align_y: Option<Vertical>,
  clip: Option<bool>,
}

impl LuaWidgetRow {
  pub fn new(inner: Vec<LuaWidget>) -> Self {
    Self {
      inner,
      spacing: None,
      padding: None,
      width: None,
      height: None,
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
    let inner_built = self
      .inner
      .iter()
      .map(|e| e.clone().build(tree, lua, path.clone(), layout_settings))
      .collect::<Vec<Element<'a, M>>>();

    let mut r = row(inner_built);

    if let Some(spacing) = self.spacing {
      r = r.spacing(spacing);
    }
    if let Some(padding) = self.padding {
      r = r.padding(padding);
    }
    if let Some(width) = self.width {
      r = r.width(width);
    }
    if let Some(height) = self.height {
      r = r.height(height);
    }
    if let Some(align_y) = self.align_y {
      r = r.align_y(align_y);
    }
    if let Some(clip) = self.clip {
      r = r.clip(clip);
    }

    r.into()
  }
}

impl FromLua for LuaWidgetRow {
  fn from_lua(value: LuaValue, _: &Lua) -> LuaResult<Self> {
    match value {
      LuaValue::UserData(ud) => Ok(ud.borrow::<Self>()?.clone()),
      _ => unreachable!(),
    }
  }
}

impl LuaUserData for LuaWidgetRow {
  fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
    methods.add_method("into", |_, w, ()| Ok(LuaWidget::Row(w.clone())));

    methods.add_method("spacing", |_, w, spacing: f32| {
      Ok(LuaWidgetRow {
        spacing: Some(Pixels(spacing)),
        ..w.clone()
      })
    });

    methods.add_method("padding", |_, w, (t, r, b, l): (f32, f32, f32, f32)| {
      Ok(LuaWidgetRow {
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
        "fill" => Ok(LuaWidgetRow {
          width: Some(Length::Fill),
          ..w.clone()
        }),
        "fill_portion" => match unit {
          Some(u) => Ok(LuaWidgetRow {
            width: Some(Length::FillPortion(u as u16)),
            ..w.clone()
          }),
          None => Err(LuaError::external(anyhow::Error::msg("missing unit"))),
        },
        "shrink" => Ok(LuaWidgetRow {
          width: Some(Length::Shrink),
          ..w.clone()
        }),
        "fixed" => match unit {
          Some(u) => Ok(LuaWidgetRow {
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
        "fill" => Ok(LuaWidgetRow {
          height: Some(Length::Fill),
          ..w.clone()
        }),
        "fill_portion" => match unit {
          Some(u) => Ok(LuaWidgetRow {
            height: Some(Length::FillPortion(u as u16)),
            ..w.clone()
          }),
          None => Err(LuaError::external(anyhow::Error::msg("missing unit"))),
        },
        "shrink" => Ok(LuaWidgetRow {
          height: Some(Length::Shrink),
          ..w.clone()
        }),
        "fixed" => match unit {
          Some(u) => Ok(LuaWidgetRow {
            height: Some(Length::Fixed(u)),
            ..w.clone()
          }),
          None => Err(LuaError::external(anyhow::Error::msg("missing unit"))),
        },
        _ => Err(LuaError::external(anyhow::Error::msg("incorrect length"))),
      },
    );

    methods.add_method("align_y", |_, w, s: String| match s.as_str() {
      "top" => Ok(LuaWidgetRow {
        align_y: Some(Vertical::Top),
        ..w.clone()
      }),
      "bottom" => Ok(LuaWidgetRow {
        align_y: Some(Vertical::Bottom),
        ..w.clone()
      }),
      "center" => Ok(LuaWidgetRow {
        align_y: Some(Vertical::Center),
        ..w.clone()
      }),
      _ => Err(LuaError::external(anyhow::Error::msg(
        "incorrect alignment",
      ))),
    });

    methods.add_method("clip", |_, w, clip: bool| {
      Ok(LuaWidgetRow {
        clip: Some(clip),
        ..w.clone()
      })
    });
  }
}

pub(super) fn init_lua_widget_row(lua: &Lua, t: &LuaTable) -> LuaResult<()> {
  let constructor = lua.create_function(|_, inner: Vec<LuaWidget>| Ok(LuaWidgetRow::new(inner)))?;
  t.set("row", constructor)?;
  Ok(())
}
