use anyhow::Result;
use handy_keys::{HotkeyId, HotkeyManager, HotkeyState};
use iced_aw::ContextMenu;
use include_dir::Dir;
use yast_core::{
  defaults::copy_default_components,
  layout::{HotkeyAction, Layout, component::Component},
  lua::{LuaContext, inject::inject_values_in_lua},
  repository::Repository,
};

#[macro_use]
extern crate log;

#[cfg(target_os = "windows")]
use iced::keyboard;
use iced::{
  Background, Color, Element, Length, Size, Subscription, Task, Theme,
  time::every,
  widget::{button, column, container, space, stack, text},
  window,
};
use livesplit_core::{
  Run, Segment, SharedTimer, Timer,
  auto_splitting::Runtime,
  run::{
    parser,
    saver::livesplit::{IoWrite, save_timer},
  },
};
use rfd::{MessageButtons, MessageDialog, MessageDialogResult};
use std::{
  collections::HashMap,
  fs::{self, File, read_to_string},
  io::BufWriter,
  path::Path,
  time::{Duration, SystemTime},
};

static VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct App {
  window_id: Option<window::Id>,
  hotkey_manager: HotkeyManager,
  hotkeys: HashMap<HotkeyId, HotkeyAction>,
  hotkeys_on: bool,
  components: HashMap<String, String>,
  lua_context: LuaContext,
  pub layout: Layout,
  repository: Repository,
  pub timer: SharedTimer,
  #[allow(unused)]
  autosplitter: Runtime,
  splits_edited: bool,
  layout_edited: bool,
}

#[derive(Clone, Debug)]
pub enum AppMessage {
  Init(Option<window::Id>),
  Update,

  WindowClosing(window::Id),
  WindowResized((window::Id, Size)),
  #[cfg(target_os = "windows")]
  KeyboardEvent(keyboard::Event),
  ResizeTimer(f32, f32),

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

impl App {
  fn new() -> (Self, Task<AppMessage>) {
    let hotkey_manager = HotkeyManager::new().expect("couldn't initialize hotkeys");
    let hotkeys = HashMap::new();

    let mut run = Run::new();
    run.push_segment(Segment::new(""));
    let timer = Timer::new(run)
      .expect("couldn't initialize timer")
      .into_shared();
    let mut repository = Repository::default();
    repository.splits_icon.push(None);

    let autosplitter = Runtime::new(timer.clone());

    let lua_context = LuaContext::init().expect("couldn't initialize lua context");

    let mut components_dir = dirs::data_dir().expect("couldn't get data directory");
    components_dir.push("yast/components");
    let components = Component::import_all_from_directory(
      &components_dir.to_string_lossy().to_string(),
      &lua_context.lua,
    )
    .expect("couldn't get components");

    (
      Self {
        window_id: None,

        hotkey_manager,
        hotkeys,
        hotkeys_on: false,

        components,
        lua_context,

        layout: Layout::default(),
        repository,

        timer,
        autosplitter,

        splits_edited: false,
        layout_edited: false,
      },
      window::latest().map(AppMessage::Init),
    )
  }

  fn update_handler(&mut self, message: AppMessage) -> Task<AppMessage> {
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

  fn update(&mut self, message: AppMessage) -> Result<Task<AppMessage>> {
    match message {
      AppMessage::Init(id) => {
        self.window_id = id;
        Ok(Task::none())
      }
      AppMessage::Update => {
        if let Some(event) = self.hotkey_manager.try_recv() {
          if let HotkeyState::Pressed = event.state {
            let mut timer = self
              .timer
              .write()
              .map_err(|_| anyhow::Error::msg("couldn't access timer"))?;
            match self
              .hotkeys
              .get(&event.id)
              .ok_or(anyhow::Error::msg(format!(
                "couldn't get hotkey {:?}",
                &event.id
              )))? {
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
          }
        }

        Ok(Task::none())
      }
      AppMessage::WindowResized((_id, size)) => {
        self.layout.width = size.width;
        self.layout.height = size.height;
        self.layout_edited = true;
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
              }
            }
            MessageDialogResult::Cancel => {
              closing = false;
            }
            _ => unreachable!(),
          }
        }

        if closing && self.layout_edited {
          let result = MessageDialog::new()
            .set_title("Save Layout?")
            .set_description("Layout hasn't been saved. Would you like to save them?")
            .set_buttons(MessageButtons::YesNoCancel)
            .show();

          match result {
            MessageDialogResult::No => {}
            MessageDialogResult::Yes => {
              let result = rfd::FileDialog::new()
                .add_filter("YAST Layout", &["yasl"])
                .save_file();
              if let Some(path) = result {
                self.layout.save(&path.to_string_lossy().to_string())?;
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
      AppMessage::ToggleHotkeys => {
        for (id, _) in self.hotkeys.drain() {
          self.hotkey_manager.unregister(id)?;
        }

        if !self.hotkeys_on {
          for (action, hotkey) in &self.layout.hotkeys {
            self.hotkeys.insert(
              self.hotkey_manager.register(hotkey.clone())?,
              action.clone(),
            );
          }
        }

        self.hotkeys_on = !self.hotkeys_on;

        Ok(Task::none())
      }
      AppMessage::LoadSplitsOpenPicker => {
        let future = Task::future(
          rfd::AsyncFileDialog::new()
            .add_filter("Compatible Splits", &["lss"])
            .pick_file(),
        )
        .then(|handle| match handle {
          Some(handle) => {
            let file_path = handle.path().to_string_lossy().to_string();
            Task::done(AppMessage::LoadSplits(file_path))
          }
          None => Task::none(),
        });
        Ok(future)
      }
      AppMessage::LoadSplits(path) => {
        let p = Path::new(&path);
        let source = fs::read(p)?;
        let parsed_run = parser::parse_and_fix(&source, Some(p))?;
        let game_name = parsed_run.run.game_name().to_string();
        let category_name = parsed_run.run.category_name().to_string();
        let timer = Timer::new(parsed_run.run)?;
        self.repository.update_from_splits(timer.run())?;
        self.timer = timer.into_shared();
        self.autosplitter = Runtime::new(self.timer.clone());
        info!("loaded splits: {} - {}", game_name, category_name);
        Ok(Task::none())
      }
      AppMessage::SaveSplitsOpenPicker => {
        let future = Task::future(
          rfd::AsyncFileDialog::new()
            .add_filter("LiveSplit Splits", &["lss"])
            .save_file(),
        )
        .then(|handle| match handle {
          Some(handle) => {
            let file_path = handle.path().to_string_lossy().to_string();
            Task::done(AppMessage::SaveSplits(file_path))
          }
          None => Task::none(),
        });
        Ok(future)
      }
      AppMessage::SaveSplits(path) => {
        self.save_splits(path)?;
        info!("saved splits");
        Ok(Task::none())
      }
      AppMessage::LoadLayoutOpenPicker => {
        let future = Task::future(
          rfd::AsyncFileDialog::new()
            .add_filter("YAST Layout", &["yasl"])
            .pick_file(),
        )
        .then(|handle| match handle {
          Some(handle) => {
            let file_path = handle.path().to_string_lossy().to_string();
            Task::done(AppMessage::LoadLayout(file_path))
          }
          None => Task::none(),
        });
        Ok(future)
      }
      AppMessage::LoadLayout(path) => {
        let toml_string = read_to_string(path)?;
        let new_layout = Layout::load(
          &mut self.repository,
          &self.components,
          &self.lua_context.lua,
          toml_string,
        )?;
        let width = new_layout.width;
        let height = new_layout.height;
        self.layout = new_layout;
        for (id, _) in self.hotkeys.drain() {
          self.hotkey_manager.unregister(id)?;
        }
        self.hotkeys_on = false;
        info!(
          "loaded layout: {} by {}",
          self.layout.name, self.layout.author
        );
        Ok(Task::done(AppMessage::ResizeTimer(width, height)))
      }
      AppMessage::SaveLayoutOpenPicker => {
        let future = Task::future(
          rfd::AsyncFileDialog::new()
            .add_filter("YAST Layout", &["yasl"])
            .save_file(),
        )
        .then(|handle| match handle {
          Some(handle) => {
            let file_path = handle.path().to_string_lossy().to_string();
            Task::done(AppMessage::SaveLayout(file_path))
          }
          None => Task::none(),
        });
        Ok(future)
      }
      AppMessage::SaveLayout(path) => {
        self.layout.save(&path)?;
        info!("saved layout");
        Ok(Task::none())
      }
      AppMessage::LoadAutosplitterOpenPicker => {
        let future = Task::future(
          rfd::AsyncFileDialog::new()
            .add_filter("LiveSplit Autosplitter", &["wasm"])
            .pick_file(),
        )
        .then(|handle| match handle {
          Some(handle) => {
            let file_path = handle.path().to_string_lossy().to_string();
            Task::done(AppMessage::LoadAutosplitter(file_path))
          }
          None => Task::none(),
        });
        Ok(future)
      }
      AppMessage::LoadAutosplitter(path) => {
        let p = Path::new(&path).to_path_buf();
        self.autosplitter.load_script_blocking(p)?;
        info!("loaded autosplitter");
        Ok(Task::none())
      }
    }
  }

  fn view(&self) -> Element<'_, AppMessage> {
    if let Ok(timer) = self.timer.read() {
      inject_values_in_lua(&self.lua_context.lua, &timer, &self.repository)
        .unwrap_or_else(|err| error!("couldn't inject values into lua: {}", err));
    }

    let inner = if let Some(lcontent) = &self.layout.content {
      lcontent
        .build(
          &self.lua_context.lua,
          vec![],
          &self.layout.settings,
          &self.repository,
        )
        .unwrap_or_else(|err| {
          error!("couldn't build layout: {}", err);
          text("couldn't build layout, please check the logs for full details")
            .width(Length::Fill)
            .height(Length::Fill)
            .center()
            .into()
        })
    } else {
      space().width(Length::Fill).height(Length::Fill).into()
    };

    let styler = |t: &Theme, _: button::Status| button::Style {
      background: Some(Background::Color(t.palette().primary)),
      text_color: t.palette().text,
      ..Default::default()
    };

    let toggle_hotkeys_state = match self.hotkeys_on {
      true => "off",
      false => "on",
    };
    let toggle_hotkeys_text = format!("toggle hotkeys {}", toggle_hotkeys_state);

    let context = ContextMenu::new(inner, move || {
      column(vec![
        button("load splits")
          .width(Length::Fill)
          .on_press(AppMessage::LoadSplitsOpenPicker)
          .style(styler)
          .into(),
        button("save splits")
          .width(Length::Fill)
          .on_press(AppMessage::SaveSplitsOpenPicker)
          .style(styler)
          .into(),
        space().width(Length::Fixed(10.0)).into(),
        button("load autosplitter")
          .width(Length::Fill)
          .on_press(AppMessage::LoadAutosplitterOpenPicker)
          .style(styler)
          .into(),
        space().width(Length::Fixed(10.0)).into(),
        button("load layout")
          .width(Length::Fill)
          .on_press(AppMessage::LoadLayoutOpenPicker)
          .style(styler)
          .into(),
        button("save layout")
          .width(Length::Fill)
          .on_press(AppMessage::SaveLayoutOpenPicker)
          .style(styler)
          .into(),
        space().width(Length::Fixed(10.0)).into(),
        button(text(toggle_hotkeys_text.clone()))
          .width(Length::Fill)
          .on_press(AppMessage::ToggleHotkeys)
          .style(styler)
          .into(),
      ])
      .width(Length::Fixed(200.))
      .spacing(2.0)
      .into()
    })
    .into();

    let stacked = stack(vec![
      container(space().width(Length::Fill).height(Length::Fill))
        .style(|_| container::Style {
          background: Some(Background::Color(Color::BLACK)),
          ..Default::default()
        })
        .into(),
      context,
    ])
    .into();

    stacked
  }

  fn title(&self) -> String {
    format!("YAST {}", VERSION)
  }

  #[cfg(not(target_os = "windows"))]
  fn subscription(&self) -> Subscription<AppMessage> {
    Subscription::batch(vec![
      window::resize_events().map(AppMessage::WindowResized),
      window::close_requests().map(AppMessage::WindowClosing),
      every(Duration::from_secs_f64(1.0 / 60.0)).map(|_| AppMessage::Update),
    ])
  }

  #[cfg(target_os = "windows")]
  fn subscription(&self) -> Subscription<AppMessage> {
    Subscription::batch(vec![
      window::resize_events().map(AppMessage::WindowResized),
      window::close_requests().map(AppMessage::WindowClosing),
      keyboard::listen().map(AppMessage::KeyboardEvent),
      every(Duration::from_secs_f64(1.0 / 60.0)).map(|_| AppMessage::Update),
    ])
  }

  fn save_splits(&self, path: String) -> Result<()> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    {
      let timer = self
        .timer
        .read()
        .map_err(|_| anyhow::Error::msg("couldn't access timer"))?;
      save_timer(&timer, IoWrite(writer))?;
    }
    Ok(())
  }
}

pub fn run_app() -> iced::Result {
  info!("starting YAST {}", VERSION);

  iced::application(App::new, App::update_handler, App::view)
    .subscription(App::subscription)
    .title(App::title)
    .theme(Theme::Dark)
    .exit_on_close_request(false)
    .run()
}

#[cfg(target_os = "macos")]
fn is_ready() -> Result<bool> {
  let acc = handy_keys::check_accessibility();
  if !acc {
    handy_keys::open_accessibility_settings()?;
  }
  acc
}

#[cfg(not(target_os = "macos"))]
fn is_ready() -> Result<bool> {
  Ok(true)
}

static DEFAULT_DIR: Dir<'_> = include_dir::include_dir!("$CARGO_MANIFEST_DIR/default");

fn main() -> Result<()> {
  fern::Dispatch::new()
    .level(log::LevelFilter::Warn)
    .level_for("yast", log::LevelFilter::Info)
    .level_for("yast_core", log::LevelFilter::Info)
    .format(move |out, message, record| {
      out.finish(format_args!(
        "[{} || {}] {} » {}",
        humantime::format_rfc3339_seconds(SystemTime::now()),
        record.level(),
        record.target(),
        message
      ))
    })
    .chain(std::io::stdout())
    .chain(fern::log_file("yast.log")?)
    .apply()?;

  copy_default_components(&DEFAULT_DIR)?;

  if is_ready()? {
    run_app()?;
  }

  Ok(())
}
