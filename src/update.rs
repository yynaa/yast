use anyhow::Result;
use handy_keys::HotkeyState;
use iced::{Size, Task, window};
use rfd::{MessageButtons, MessageDialog, MessageDialogResult};
use yast_core::layout::HotkeyAction;

#[cfg(target_os = "windows")]
use iced::keyboard;

use crate::{App, AppMessage, menu::Menu};

impl App {
  pub fn update_handler(&mut self, message: AppMessage) -> Task<AppMessage> {
    match &message {
      AppMessage::Update => {}
      anything => {
        trace!("action: {:?}", anything);
      }
    }

    self.update(message.clone()).unwrap_or_else(|err| {
      error!(
        "error occured updating message {:?}: {}",
        message.clone(),
        err
      );
      Task::none()
    })
  }

  /// common function for handling hotkeys
  ///
  /// used by windows-only calls and the regular handykeys callback
  pub fn handle_hotkey(&mut self, action: HotkeyAction) -> Result<()> {
    let mut timer = self
      .timer
      .write()
      .map_err(|_| anyhow::Error::msg("couldn't access timer"))?;

    match action {
      HotkeyAction::StartOrSplitTimer => {
        timer.split_or_start();
      }
      HotkeyAction::StartTimer => {
        timer.start();
      }
      HotkeyAction::SplitTimer => {
        timer.split();
      }
      HotkeyAction::ResetTimerWithoutSaving => {
        timer.reset(false);
      }
      HotkeyAction::ResetTimer => {
        self.splits_edited = true;
        timer.reset(true);
      }
      HotkeyAction::SkipSplit => {
        timer.skip_split();
      }
      HotkeyAction::UndoSplit => {
        timer.undo_split();
      }
      HotkeyAction::PauseTimer => {
        timer.toggle_pause();
      }
      HotkeyAction::ToggleTimingMethod => {
        timer.toggle_timing_method();
      }
      HotkeyAction::NextComparison => {
        timer.switch_to_next_comparison();
      }
      HotkeyAction::PreviousComparison => {
        timer.switch_to_previous_comparison();
      }
    }

    Ok(())
  }

  pub fn update(&mut self, message: AppMessage) -> Result<Task<AppMessage>> {
    match message {
      AppMessage::Init(id) => {
        self.window_id = id;
        Ok(Task::none())
      }
      AppMessage::Update => {
        if let Some(event) = self.hotkey_manager.try_recv() {
          if let HotkeyState::Pressed = event.state {
            let hotkey = self
              .hotkeys
              .get(&event.id)
              .ok_or(anyhow::Error::msg(format!(
                "couldn't get hotkey {:?}",
                &event.id
              )))?
              .clone();
            self.handle_hotkey(hotkey)?;
          }
        }

        Ok(Task::none())
      }
      AppMessage::WindowResized((_id, size)) => {
        self.layout.width = size.width;
        self.layout.height = size.height;
        Ok(Task::none())
      }
      AppMessage::WindowClosing(_id) => {
        let mut task = Task::none();
        let mut closing = true;

        if closing && self.splits_edited {
          let result = MessageDialog::new()
            .set_title("Save Splits?")
            .set_description("Splits haven't been saved. Would you like to save them?")
            .set_buttons(MessageButtons::YesNoCancel)
            .show();

          match result {
            MessageDialogResult::No => {}
            MessageDialogResult::Yes => {
              let result = rfd::FileDialog::new()
                .add_filter("LiveSplit Splits", &["lss"])
                .save_file();
              if let Some(path) = result {
                self.save_splits(path.to_string_lossy().to_string())?;
                self.splits_edited = false;
              }
            }
            MessageDialogResult::Cancel => {
              closing = false;
            }
            _ => unreachable!(),
          }
        }

        if closing {
          task = task.chain(iced::exit());
        }

        Ok(task)
      }
      #[cfg(target_os = "windows")]
      AppMessage::KeyboardEvent(event) => {
        use yast_windows;
        if let keyboard::Event::KeyPressed {
          key: _,
          modified_key: _,
          physical_key: _,
          location: _,
          modifiers: _,
          text: _,
          repeat: _,
        } = event
        {
          if let Some(translated_hotkey) = yast_windows::translate_event_to_hotkey(event)? {
            for (action, hotkey) in &self.layout.hotkeys {
              if *hotkey == translated_hotkey {
                self.handle_hotkey(action.clone())?;
                break;
              }
            }
          }
        }

        Ok(Task::none())
      }
      AppMessage::ResizeTimer(w, h) => Ok(window::resize(
        self.window_id.expect("no window id stored in app"),
        Size::new(w, h),
      )),
      AppMessage::MenuMessage(msg) => Menu::update(self, msg),
    }
  }
}
