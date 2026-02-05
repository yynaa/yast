use crate::layout::component::Component;

pub mod component;

// serializing this may involve something like https://github.com/dtolnay/typetag
pub struct Layout {
  pub name: String,
  pub author: String,
  pub content: Option<Component>,

  pub width: f32,
  pub height: f32,
}

impl Default for Layout {
  fn default() -> Self {
    Self {
      name: String::from("untitled"),
      author: String::new(),
      content: None,

      width: 200.,
      height: 500.,
    }
  }
}
