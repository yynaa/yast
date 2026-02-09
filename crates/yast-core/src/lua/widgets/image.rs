use iced::{ContentFit, Element, Length, Rectangle, widget::image};
use mlua::prelude::*;

use crate::lua::widgets::LuaWidget;

#[derive(Clone)]
pub struct ImageHandleLua(pub image::Handle);

impl FromLua for ImageHandleLua {
  fn from_lua(value: LuaValue, _: &Lua) -> LuaResult<Self> {
    match value {
      LuaValue::UserData(ud) => Ok(ud.borrow::<Self>()?.clone()),
      _ => unreachable!(),
    }
  }
}

impl LuaUserData for ImageHandleLua {}

#[derive(Clone)]
pub struct LuaWidgetImage {
  handle: ImageHandleLua,
  width: Option<Length>,
  height: Option<Length>,
  content_fit: Option<ContentFit>,
  filter_method: Option<image::FilterMethod>,
  opacity: Option<f32>,
  crop: Option<Rectangle<u32>>,
}

impl LuaWidgetImage {
  pub fn new(handle: ImageHandleLua) -> Self {
    Self {
      handle,
      width: None,
      height: None,
      content_fit: None,
      filter_method: None,
      opacity: None,
      crop: None,
    }
  }

  pub fn build<'a, M>(self) -> Element<'a, M> {
    let mut img = image(self.handle.0.clone());

    if let Some(width) = self.width {
      img = img.width(width);
    }
    if let Some(height) = self.height {
      img = img.height(height);
    }
    if let Some(content_fit) = self.content_fit {
      img = img.content_fit(content_fit);
    }
    if let Some(filter_method) = self.filter_method {
      img = img.filter_method(filter_method);
    }
    if let Some(opacity) = self.opacity {
      img = img.opacity(opacity);
    }
    if let Some(crop) = self.crop {
      img = img.crop(crop);
    }

    img.into()
  }
}

impl FromLua for LuaWidgetImage {
  fn from_lua(value: LuaValue, _: &Lua) -> LuaResult<Self> {
    match value {
      LuaValue::UserData(ud) => Ok(ud.borrow::<Self>()?.clone()),
      _ => unreachable!(),
    }
  }
}

impl LuaUserData for LuaWidgetImage {
  fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
    methods.add_method("into", |_, w, ()| Ok(LuaWidget::Image(w.clone())));

    methods.add_method(
      "width",
      |_, w, (typ, unit): (String, Option<f32>)| match typ.as_str() {
        "fill" => Ok(LuaWidgetImage {
          width: Some(Length::Fill),
          ..w.clone()
        }),
        "fill_portion" => match unit {
          Some(u) => Ok(LuaWidgetImage {
            width: Some(Length::FillPortion(u as u16)),
            ..w.clone()
          }),
          None => Err(LuaError::external(anyhow::Error::msg("missing unit"))),
        },
        "shrink" => Ok(LuaWidgetImage {
          width: Some(Length::Shrink),
          ..w.clone()
        }),
        "fixed" => match unit {
          Some(u) => Ok(LuaWidgetImage {
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
        "fill" => Ok(LuaWidgetImage {
          height: Some(Length::Fill),
          ..w.clone()
        }),
        "fill_portion" => match unit {
          Some(u) => Ok(LuaWidgetImage {
            height: Some(Length::FillPortion(u as u16)),
            ..w.clone()
          }),
          None => Err(LuaError::external(anyhow::Error::msg("missing unit"))),
        },
        "shrink" => Ok(LuaWidgetImage {
          height: Some(Length::Shrink),
          ..w.clone()
        }),
        "fixed" => match unit {
          Some(u) => Ok(LuaWidgetImage {
            height: Some(Length::Fixed(u)),
            ..w.clone()
          }),
          None => Err(LuaError::external(anyhow::Error::msg("missing unit"))),
        },
        _ => Err(LuaError::external(anyhow::Error::msg("incorrect length"))),
      },
    );

    methods.add_method("content_fit", |_, w, s: String| match s.as_str() {
      "contain" => Ok(LuaWidgetImage {
        content_fit: Some(ContentFit::Contain),
        ..w.clone()
      }),
      "cover" => Ok(LuaWidgetImage {
        content_fit: Some(ContentFit::Cover),
        ..w.clone()
      }),
      "fill" => Ok(LuaWidgetImage {
        content_fit: Some(ContentFit::Fill),
        ..w.clone()
      }),
      "none" => Ok(LuaWidgetImage {
        content_fit: Some(ContentFit::None),
        ..w.clone()
      }),
      "scale_down" => Ok(LuaWidgetImage {
        content_fit: Some(ContentFit::ScaleDown),
        ..w.clone()
      }),
      _ => Err(LuaError::external(anyhow::Error::msg(
        "incorrect content_fit",
      ))),
    });

    methods.add_method("filter_method", |_, w, s: String| match s.as_str() {
      "linear" => Ok(LuaWidgetImage {
        filter_method: Some(image::FilterMethod::Linear),
        ..w.clone()
      }),
      "nearest" => Ok(LuaWidgetImage {
        filter_method: Some(image::FilterMethod::Nearest),
        ..w.clone()
      }),
      _ => Err(LuaError::external(anyhow::Error::msg(
        "incorrect filter_method",
      ))),
    });

    methods.add_method("opacity", |_, w, opacity: f32| {
      Ok(LuaWidgetImage {
        opacity: Some(opacity),
        ..w.clone()
      })
    });

    methods.add_method(
      "crop",
      |_, w, (x, y, width, height): (u32, u32, u32, u32)| {
        Ok(LuaWidgetImage {
          crop: Some(Rectangle {
            x,
            y,
            width,
            height,
          }),
          ..w.clone()
        })
      },
    );
  }
}

pub(super) fn init_lua_widget_image(lua: &Lua, t: &LuaTable) -> LuaResult<()> {
  let constructor =
    lua.create_function(|_, handle: ImageHandleLua| Ok(LuaWidgetImage::new(handle)))?;
  t.set("image", constructor)?;
  Ok(())
}
