use anyhow::Result;

use crate::layout::{Layout, settings::LayoutSettings};

impl Layout {
  fn settings_move_from(
    &mut self,
    old: &LayoutSettings,
    old_prefix: Vec<usize>,
    new_prefix: Vec<usize>,
  ) -> Result<()> {
    for (path, content) in old {
      if path.starts_with(&old_prefix) {
        let mut new_path = new_prefix.clone();
        new_path.extend(path.split_at(old_prefix.len()).1);
        self.settings.insert(new_path, content.clone());
      }
    }

    Ok(())
  }

  fn settings_swap(&mut self, a: Vec<usize>, b: Vec<usize>) -> Result<()> {
    let old = self.settings.clone();

    self.settings_move_from(&old, a.clone(), b.clone())?;
    self.settings_move_from(&old, b, a)?;

    Ok(())
  }

  fn settings_move_up(&mut self, mut path: Vec<usize>) -> Result<Vec<usize>> {
    let last = path
      .pop()
      .ok_or(anyhow::Error::msg("cannot move root component settings"))?;

    if last > 0 {
      let mut a = path.clone();
      a.push(last);
      let mut b = path.clone();
      b.push(last - 1);
      self.settings_swap(a, b)?;
      path.push(last - 1);
      Ok(path)
    } else {
      path.push(last);
      Ok(path)
    }
  }

  fn settings_move_down(&mut self, mut path: Vec<usize>) -> Result<Vec<usize>> {
    let last = path
      .pop()
      .ok_or(anyhow::Error::msg("cannot move root component settings"))?;
    let parent_component_length = self
      .content
      .as_ref()
      .unwrap()
      .get_from_path(path.clone())?
      .children
      .len();

    if last < parent_component_length - 1 {
      let mut a = path.clone();
      a.push(last);
      let mut b = path.clone();
      b.push(last + 1);
      self.settings_swap(a, b)?;
      path.push(last + 1);
      Ok(path)
    } else {
      path.push(last);
      Ok(path)
    }
  }

  fn tree_move_up(&mut self, mut path: Vec<usize>) -> Result<Vec<usize>> {
    let last = path
      .pop()
      .ok_or(anyhow::Error::msg("cannot move root component settings"))?;
    if last > 0 {
      let parent = self
        .content
        .as_mut()
        .unwrap()
        .get_mut_from_path(path.clone())?;
      let removed = parent.children.remove(last);
      parent.children.insert(last - 1, removed);
      path.push(last - 1);
      Ok(path)
    } else {
      path.push(last);
      Ok(path)
    }
  }

  fn tree_move_down(&mut self, mut path: Vec<usize>) -> Result<Vec<usize>> {
    let last = path
      .pop()
      .ok_or(anyhow::Error::msg("cannot move root component settings"))?;
    let parent = self
      .content
      .as_mut()
      .unwrap()
      .get_mut_from_path(path.clone())?;

    if last < parent.children.len() - 1 {
      let removed = parent.children.remove(last);
      parent.children.insert(last + 1, removed);
      path.push(last + 1);
      Ok(path)
    } else {
      path.push(last);
      Ok(path)
    }
  }

  pub fn component_move_up(&mut self, path: Vec<usize>) -> Result<Vec<usize>> {
    let s = self.settings_move_up(path.clone()).map_err(|err| {
      anyhow::Error::msg(format!(
        "couldn't move settings up (corrupted data!): {}",
        err
      ))
    })?;
    let t = self.tree_move_up(path.clone()).map_err(|err| {
      anyhow::Error::msg(format!("couldn't move tree up (corrupted data!): {}", err))
    })?;
    if s == t {
      Ok(s)
    } else {
      Err(anyhow::Error::msg(
        "not both settings and tree were moved up (corrupted data!)",
      ))
    }
  }

  pub fn component_move_down(&mut self, path: Vec<usize>) -> Result<Vec<usize>> {
    let s = self.settings_move_down(path.clone()).map_err(|err| {
      anyhow::Error::msg(format!(
        "couldn't move settings down (corrupted data!): {}",
        err
      ))
    })?;
    let t = self.tree_move_down(path.clone()).map_err(|err| {
      anyhow::Error::msg(format!(
        "couldn't move tree down (corrupted data!): {}",
        err
      ))
    })?;
    if s == t {
      Ok(s)
    } else {
      Err(anyhow::Error::msg(
        "not both settings and tree were moved down (corrupted data!)",
      ))
    }
  }
}
