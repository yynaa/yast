use std::{
  fs::{self, read_to_string},
  path::Path,
};

use anyhow::Result;
use iced::{
  Background, Color, Element, Length, Task,
  alignment::Horizontal,
  widget::{button, column, container, opaque, row, space, stack, text},
};
use livesplit_core::{Timer, auto_splitting::Runtime, run::parser};
use yast_core::layout::Layout;

use crate::{App, AppMessage};

pub struct Menu {
  pub opened: bool,
}

#[derive(Clone, Debug)]
pub enum MenuMessage {
  ToggleMenu,

  ToggleHotkeys,
  LoadSplitsOpenPicker,
  LoadSplits(String),
  SaveSplitsOpenPicker,
  SaveSplits(String),
  LoadLayoutOpenPicker,
  LoadLayout(String),
  SaveLayoutOpenPicker,
  SaveLayout(String),
  LoadAutosplitterOpenPicker,
  LoadAutosplitter(String),
}

impl Menu {
  pub fn new() -> Self {
    Self { opened: false }
  }

  pub fn update(app: &mut App, message: MenuMessage) -> Result<Task<AppMessage>> {
    match message {
      MenuMessage::ToggleMenu => {
        app.menu.opened = !app.menu.opened;
        Ok(Task::none())
      }
      MenuMessage::ToggleHotkeys => {
        for (id, _) in app.hotkeys.drain() {
          app.hotkey_manager.unregister(id)?;
        }

        if !app.hotkeys_on {
          for (action, hotkey) in &app.layout.hotkeys {
            app
              .hotkeys
              .insert(app.hotkey_manager.register(hotkey.clone())?, action.clone());
          }
        }

        app.hotkeys_on = !app.hotkeys_on;

        Ok(Task::none())
      }
      MenuMessage::LoadSplitsOpenPicker => {
        let future = Task::future(
          rfd::AsyncFileDialog::new()
            .add_filter("Compatible Splits", &["lss"])
            .pick_file(),
        )
        .then(|handle| match handle {
          Some(handle) => {
            let file_path = handle.path().to_string_lossy().to_string();
            Task::done(AppMessage::MenuMessage(MenuMessage::LoadSplits(file_path)))
          }
          None => Task::none(),
        });
        Ok(future)
      }
      MenuMessage::LoadSplits(path) => {
        let p = Path::new(&path);
        let source = fs::read(p)?;
        let parsed_run = parser::parse_and_fix(&source, Some(p))?;
        let game_name = parsed_run.run.game_name().to_string();
        let category_name = parsed_run.run.category_name().to_string();
        let timer = Timer::new(parsed_run.run)?;
        app.repository.update_from_splits(timer.run())?;
        app.timer = timer.into_shared();
        app.autosplitter = Runtime::new(app.timer.clone());
        info!("loaded splits: {} - {}", game_name, category_name);
        Ok(Task::none())
      }
      MenuMessage::SaveSplitsOpenPicker => {
        let future = Task::future(
          rfd::AsyncFileDialog::new()
            .add_filter("LiveSplit Splits", &["lss"])
            .save_file(),
        )
        .then(|handle| match handle {
          Some(handle) => {
            let file_path = handle.path().to_string_lossy().to_string();
            Task::done(AppMessage::MenuMessage(MenuMessage::SaveSplits(file_path)))
          }
          None => Task::none(),
        });
        Ok(future)
      }
      MenuMessage::SaveSplits(path) => {
        app.save_splits(path)?;
        info!("saved splits");
        Ok(Task::none())
      }
      MenuMessage::LoadLayoutOpenPicker => {
        let future = Task::future(
          rfd::AsyncFileDialog::new()
            .add_filter("YAST Layout", &["yasl"])
            .pick_file(),
        )
        .then(|handle| match handle {
          Some(handle) => {
            let file_path = handle.path().to_string_lossy().to_string();
            Task::done(AppMessage::MenuMessage(MenuMessage::LoadLayout(file_path)))
          }
          None => Task::none(),
        });
        Ok(future)
      }
      MenuMessage::LoadLayout(path) => {
        let toml_string = read_to_string(path)?;
        let new_layout = Layout::load(
          &mut app.repository,
          &app.components,
          &app.lua_context.lua,
          toml_string,
        )?;
        let width = new_layout.width;
        let height = new_layout.height;
        app.layout = new_layout;
        for (id, _) in app.hotkeys.drain() {
          app.hotkey_manager.unregister(id)?;
        }
        app.hotkeys_on = false;
        info!(
          "loaded layout: {} by {}",
          app.layout.name, app.layout.author
        );
        Ok(Task::done(AppMessage::ResizeTimer(width, height)))
      }
      MenuMessage::SaveLayoutOpenPicker => {
        let future = Task::future(
          rfd::AsyncFileDialog::new()
            .add_filter("YAST Layout", &["yasl"])
            .save_file(),
        )
        .then(|handle| match handle {
          Some(handle) => {
            let file_path = handle.path().to_string_lossy().to_string();
            Task::done(AppMessage::MenuMessage(MenuMessage::SaveLayout(file_path)))
          }
          None => Task::none(),
        });
        Ok(future)
      }
      MenuMessage::SaveLayout(path) => {
        app.layout.save(&path)?;
        info!("saved layout");
        Ok(Task::none())
      }
      MenuMessage::LoadAutosplitterOpenPicker => {
        let future = Task::future(
          rfd::AsyncFileDialog::new()
            .add_filter("LiveSplit Autosplitter", &["wasm"])
            .pick_file(),
        )
        .then(|handle| match handle {
          Some(handle) => {
            let file_path = handle.path().to_string_lossy().to_string();
            Task::done(AppMessage::MenuMessage(MenuMessage::LoadAutosplitter(
              file_path,
            )))
          }
          None => Task::none(),
        });
        Ok(future)
      }
      MenuMessage::LoadAutosplitter(path) => {
        let p = Path::new(&path).to_path_buf();
        app.autosplitter.load_script_blocking(p)?;
        info!("loaded autosplitter");
        Ok(Task::none())
      }
    }
  }

  pub fn view(app: &App) -> Element<'_, AppMessage> {
    let mut children = Vec::new();

    children.push(
      button("Close Menu")
        .on_press(AppMessage::MenuMessage(MenuMessage::ToggleMenu))
        .style(button::danger)
        .into(),
    );

    children.push(
      row(vec![
        button("Load Splits")
          .on_press(AppMessage::MenuMessage(MenuMessage::LoadSplitsOpenPicker))
          .into(),
        button("Save Splits")
          .on_press(AppMessage::MenuMessage(MenuMessage::SaveSplitsOpenPicker))
          .style(button::secondary)
          .into(),
      ])
      .spacing(5.)
      .into(),
    );

    children.push(
      row(vec![
        button("Load Layout")
          .on_press(AppMessage::MenuMessage(MenuMessage::LoadLayoutOpenPicker))
          .into(),
        button("Save Layout")
          .on_press(AppMessage::MenuMessage(MenuMessage::SaveLayoutOpenPicker))
          .style(button::secondary)
          .into(),
      ])
      .spacing(5.)
      .into(),
    );

    children.push(
      button("Load Autosplitter")
        .on_press(AppMessage::MenuMessage(
          MenuMessage::LoadAutosplitterOpenPicker,
        ))
        .style(button::warning)
        .into(),
    );

    let mut hotkey_button =
      button("Toggle Hotkeys").on_press(AppMessage::MenuMessage(MenuMessage::ToggleHotkeys));
    if app.hotkeys_on {
      hotkey_button = hotkey_button.style(button::success);
    } else {
      hotkey_button = hotkey_button.style(button::danger);
    }
    children.push(hotkey_button.into());

    for (action, hotkey) in &app.layout.hotkeys {
      children.push(text(format!("{:?}: {}", action, hotkey)).size(8.).into());
    }

    let content = stack(vec![
      container(space().width(Length::Fill).height(Length::Fill))
        .style(|_| container::Style {
          background: Some(Background::Color(Color::from_rgba(0., 0., 0., 0.5))),
          ..Default::default()
        })
        .into(),
      column(children)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(10.)
        .spacing(5.)
        .align_x(Horizontal::Center)
        .into(),
    ]);

    let opaque = opaque(content).into();

    opaque
  }
}
