use crate::layout::LayoutPart;

pub struct LayoutColumn {
  inner: Vec<Box<dyn LayoutPart>>,
}
