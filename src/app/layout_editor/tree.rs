use iced::{
  Element, Length,
  widget::{button, text},
};

use crate::{
  app::{AppMessage, layout_editor::LayoutEditorMessage},
  layout::LayoutPart,
};

pub fn build_tree_from_layout_part<'a>(
  p: &Box<dyn LayoutPart>,
  path: Vec<usize>,
) -> Vec<Element<'a, AppMessage>> {
  let mut name = p.get_name();
  for _ in 0..path.len() {
    name = format!("  {}", name);
  }
  // I DON T KNOW WHY I HAVE TO DO THIS STUPID VARIABLE !!! BORROW CHECKERED
  let final_name = name.clone();

  let mut r = Vec::new();
  // why do i need to put text in the button? who god damn knows
  r.push(
    button(text(final_name))
      .on_press(AppMessage::LayoutEditor(
        LayoutEditorMessage::OpenComponent(path.clone()),
      ))
      .width(Length::Fill)
      .into(),
  );

  if let Some(c) = p.get_children() {
    for (i, b) in c.iter().enumerate() {
      let mut new_path = path.clone();
      new_path.push(i);

      r.append(&mut build_tree_from_layout_part(b, new_path));
    }
  }

  r
}
