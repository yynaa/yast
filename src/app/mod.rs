use std::{
  collections::{BTreeMap, HashMap},
  time::Duration,
};

use iced::{Element, Subscription, Task, Theme, time::every, widget::space, window};
use livesplit_core::{HotkeySystem, Run, Segment, SharedTimer, Timer as LSTimer};

use crate::{
  app::{
    layout_editor::{LayoutEditor, LayoutEditorMessage},
    timer::{Timer, TimerMessage},
  },
  layout::{Layout, component::Component},
  lua::LuaAppContext,
};

mod layout_editor;
mod timer;

pub trait Window {
  fn title(&self) -> String;
  fn update(&mut self, context: &mut AppContext, message: AppMessage) -> Task<AppMessage>;
  fn view(&self, context: &AppContext) -> Element<'_, AppMessage>;
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum WindowType {
  Timer,
  LayoutEditor,
}

pub struct App {
  window_ids: BTreeMap<window::Id, WindowType>,
  windows: BTreeMap<WindowType, Box<dyn Window>>,

  context: AppContext,
}

pub struct AppContext {
  components: HashMap<String, String>,
  lua_context: LuaAppContext,

  pub layout: Layout,

  pub timer: LSTimer,
}

#[derive(Clone, Debug)]
pub enum AppMessage {
  WindowClosed(window::Id),
  OpenTimer(window::Id),
  RequestLayoutEditor,
  OpenLayoutEditor(window::Id),
  UpdateView,

  Timer(TimerMessage),
  LayoutEditor(LayoutEditorMessage),
}

impl App {
  fn new() -> (Self, Task<AppMessage>) {
    let mut run = Run::new();
    run.push_segment(Segment::new(""));
    let timer = LSTimer::new(run).unwrap();

    let lua_context = LuaAppContext::init().expect("couldn't initialize lua context");

    (
      Self {
        window_ids: BTreeMap::new(),
        windows: BTreeMap::new(),

        context: AppContext {
          components: Component::import_all_from_directory("components/", &lua_context.lua)
            .expect("couldn't get components"),
          lua_context,

          layout: Layout::default(),

          timer,
        },
      },
      Timer::open_window().map(AppMessage::OpenTimer),
    )
  }

  fn update(&mut self, message: AppMessage) -> Task<AppMessage> {
    match message {
      AppMessage::WindowClosed(id) => {
        if let Some(typ) = self.window_ids.remove(&id) {
          self.windows.remove(&typ);
        }

        if self.windows.is_empty() {
          iced::exit()
        } else {
          Task::none()
        }
      }
      AppMessage::OpenTimer(id) => {
        let timer = Timer::new();
        self.window_ids.insert(id, WindowType::Timer);
        self.windows.insert(WindowType::Timer, Box::new(timer));
        Task::none()
      }
      AppMessage::RequestLayoutEditor => {
        LayoutEditor::open_window().map(AppMessage::OpenLayoutEditor)
      }
      AppMessage::OpenLayoutEditor(id) => {
        let le = LayoutEditor::new(&self.context);
        self.window_ids.insert(id, WindowType::LayoutEditor);
        self.windows.insert(WindowType::LayoutEditor, Box::new(le));
        Task::none()
      }
      AppMessage::Timer(_) => {
        if let Some(inner) = self.windows.get_mut(&WindowType::Timer) {
          inner.update(&mut self.context, message)
        } else {
          Task::none()
        }
      }
      AppMessage::LayoutEditor(_) => {
        if let Some(inner) = self.windows.get_mut(&WindowType::LayoutEditor) {
          inner.update(&mut self.context, message)
        } else {
          Task::none()
        }
      }
      _ => Task::none(),
    }
  }

  fn view(&self, window_id: window::Id) -> Element<'_, AppMessage> {
    if let Some(window_type) = self.window_ids.get(&window_id) {
      if let Some(window) = self.windows.get(&window_type) {
        return window.view(&self.context);
      }
    }
    space().into()
  }

  fn title(&self, window_id: window::Id) -> String {
    if let Some(window_type) = self.window_ids.get(&window_id) {
      if let Some(window) = self.windows.get(&window_type) {
        return window.title();
      }
    }
    String::from("YAST")
  }

  fn subscription(&self) -> Subscription<AppMessage> {
    Subscription::batch(vec![
      window::close_events().map(AppMessage::WindowClosed),
      every(Duration::from_secs_f64(1.0 / 30.0)).map(|_| AppMessage::UpdateView),
    ])
  }
}

pub fn run_app() -> iced::Result {
  info!("starting YAST app");

  iced::daemon(App::new, App::update, App::view)
    .subscription(App::subscription)
    .title(App::title)
    .theme(Theme::Dark)
    .run()
}
