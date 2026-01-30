use anyhow::Result;
use iced::{
  Color, Element, Length, Theme,
  widget::{button, text},
};

use crate::{
  app::{AppMessage, layout_editor::LayoutEditorMessage},
  layout::LayoutPart,
};

pub fn get_mut_component_at_path(
  p: &mut Box<dyn LayoutPart>,
  mut path: Vec<usize>,
) -> Result<&mut Box<dyn LayoutPart>> {
  if path.len() > 0 {
    let popped = path.remove(0);
    let child = p
      .get_children_mut()
      .ok_or(anyhow::Error::msg("invalid path (no children)"))?
      .get_mut(popped)
      .ok_or(anyhow::Error::msg("invalid path (no such child at index)"))?;
    get_mut_component_at_path(child, path)
  } else {
    Ok(p)
  }
}

pub fn build_tree_from_layout_part<'a>(
  p: &Box<dyn LayoutPart>,
  path: Vec<usize>,
  current_path: &Vec<usize>,
) -> Vec<Element<'a, AppMessage>> {
  let mut name = p.get_name();
  for _ in 0..path.len() {
    name = format!("  {}", name);
  }
  // I DON T KNOW WHY I HAVE TO DO THIS STUPID VARIABLE !!! BORROW CHECKERED
  let final_name = name.clone();

  let mut r = Vec::new();
  let is_current = *current_path == path;
  r.push(
    button(text(final_name))
      .on_press(AppMessage::LayoutEditor(
        LayoutEditorMessage::OpenComponent(path.clone()),
      ))
      .style(move |t: &Theme, _| button::Style {
        background: match is_current {
          true => Some(iced::Background::Color(t.palette().primary)),
          false => None,
        },
        text_color: Color::WHITE,
        ..Default::default()
      })
      .width(Length::Fill)
      .into(),
  );

  if let Some(c) = p.get_children() {
    for (i, b) in c.iter().enumerate() {
      let mut new_path = path.clone();
      new_path.push(i);

      r.append(&mut build_tree_from_layout_part(b, new_path, current_path));
    }
  }

  r
}
