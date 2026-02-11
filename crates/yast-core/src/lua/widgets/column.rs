use iced::{Element, Length, Padding, Pixels, alignment::Horizontal, widget::column};
use mlua::prelude::*;

use crate::{
  layout::{component::Component, settings::LayoutSettings},
  lua::widgets::LuaWidget,
  repository::Repository,
};

#[derive(Clone)]
pub struct LuaWidgetColumn {
  inner: Vec<LuaWidget>,
  spacing: Option<Pixels>,
  padding: Option<Padding>,
  width: Option<Length>,
  height: Option<Length>,
  align_x: Option<Horizontal>,
  clip: Option<bool>,
}

impl LuaWidgetColumn {
  pub fn new(inner: Vec<LuaWidget>) -> Self {
    Self {
      inner,
      spacing: None,
      padding: None,
      width: None,
      height: None,
      align_x: None,
      clip: None,
    }
  }

  pub fn build<'a, M: 'a>(
    self,
    tree: &Component,
    lua: &Lua,
    path: Vec<usize>,
    layout_settings: &LayoutSettings,
    repository: &Repository,
  ) -> Element<'a, M> {
    let inner_built = self
      .inner
      .iter()
      .map(|e| {
        e.clone()
          .build(tree, lua, path.clone(), layout_settings, repository)
      })
      .collect::<Vec<Element<'a, M>>>();

    let mut c = column(inner_built);

    if let Some(spacing) = self.spacing {
      c = c.spacing(spacing);
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
    if let Some(clip) = self.clip {
      c = c.clip(clip);
    }

    c.into()
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

    methods.add_method("spacing", |_, w, spacing: f32| {
      Ok(LuaWidgetColumn {
        spacing: Some(Pixels(spacing)),
        ..w.clone()
      })
    });

    methods.add_method("padding", |_, w, (t, r, b, l): (f32, f32, f32, f32)| {
      Ok(LuaWidgetColumn {
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
        "fill" => Ok(LuaWidgetColumn {
          width: Some(Length::Fill),
          ..w.clone()
        }),
        "fill_portion" => match unit {
          Some(u) => Ok(LuaWidgetColumn {
            width: Some(Length::FillPortion(u as u16)),
            ..w.clone()
          }),
          None => Err(LuaError::external(anyhow::Error::msg("missing unit"))),
        },
        "shrink" => Ok(LuaWidgetColumn {
          width: Some(Length::Shrink),
          ..w.clone()
        }),
        "fixed" => match unit {
          Some(u) => Ok(LuaWidgetColumn {
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
        "fill" => Ok(LuaWidgetColumn {
          height: Some(Length::Fill),
          ..w.clone()
        }),
        "fill_portion" => match unit {
          Some(u) => Ok(LuaWidgetColumn {
            height: Some(Length::FillPortion(u as u16)),
            ..w.clone()
          }),
          None => Err(LuaError::external(anyhow::Error::msg("missing unit"))),
        },
        "shrink" => Ok(LuaWidgetColumn {
          height: Some(Length::Shrink),
          ..w.clone()
        }),
        "fixed" => match unit {
          Some(u) => Ok(LuaWidgetColumn {
            height: Some(Length::Fixed(u)),
            ..w.clone()
          }),
          None => Err(LuaError::external(anyhow::Error::msg("missing unit"))),
        },
        _ => Err(LuaError::external(anyhow::Error::msg("incorrect length"))),
      },
    );

    methods.add_method("align_x", |_, w, s: String| match s.as_str() {
      "left" => Ok(LuaWidgetColumn {
        align_x: Some(Horizontal::Left),
        ..w.clone()
      }),
      "right" => Ok(LuaWidgetColumn {
        align_x: Some(Horizontal::Right),
        ..w.clone()
      }),
      "center" => Ok(LuaWidgetColumn {
        align_x: Some(Horizontal::Center),
        ..w.clone()
      }),
      _ => Err(LuaError::external(anyhow::Error::msg(
        "incorrect alignment",
      ))),
    });

    methods.add_method("clip", |_, w, clip: bool| {
      Ok(LuaWidgetColumn {
        clip: Some(clip),
        ..w.clone()
      })
    });
  }
}

pub(super) fn init_lua_widget_column(lua: &Lua, t: &LuaTable) -> LuaResult<()> {
  let constructor =
    lua.create_function(|_, inner: Vec<LuaWidget>| Ok(LuaWidgetColumn::new(inner)))?;
  t.set("column", constructor)?;
  Ok(())
}
