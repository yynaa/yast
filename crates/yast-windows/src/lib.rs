use anyhow::Result;

use handy_keys::{Hotkey, Key as HandyKey, Modifiers as HandyModifiers};
use iced::keyboard::{Event, Key as IcedKey, Modifiers as IcedModifiers, key::Named};

fn translate_key_to_key(key: IcedKey) -> Option<HandyKey> {
  match key {
    IcedKey::Unidentified => None,
    IcedKey::Named(n) => match n {
      Named::CapsLock => Some(HandyKey::CapsLock),
      Named::NumLock => Some(HandyKey::NumLock),
      Named::ScrollLock => Some(HandyKey::ScrollLock),
      Named::Enter => Some(HandyKey::Return),
      Named::Tab => Some(HandyKey::Tab),
      Named::Space => Some(HandyKey::Space),
      Named::ArrowDown => Some(HandyKey::DownArrow),
      Named::ArrowLeft => Some(HandyKey::LeftArrow),
      Named::ArrowRight => Some(HandyKey::RightArrow),
      Named::ArrowUp => Some(HandyKey::UpArrow),
      Named::End => Some(HandyKey::End),
      Named::Home => Some(HandyKey::Home),
      Named::PageDown => Some(HandyKey::PageDown),
      Named::PageUp => Some(HandyKey::PageUp),
      Named::Backspace => Some(HandyKey::Delete),
      Named::Delete => Some(HandyKey::ForwardDelete),
      Named::Escape => Some(HandyKey::Escape),
      Named::F1 => Some(HandyKey::F1),
      Named::F2 => Some(HandyKey::F2),
      Named::F3 => Some(HandyKey::F3),
      Named::F4 => Some(HandyKey::F4),
      Named::F5 => Some(HandyKey::F5),
      Named::F6 => Some(HandyKey::F6),
      Named::F7 => Some(HandyKey::F7),
      Named::F8 => Some(HandyKey::F8),
      Named::F9 => Some(HandyKey::F9),
      Named::F10 => Some(HandyKey::F10),
      Named::F11 => Some(HandyKey::F11),
      Named::F12 => Some(HandyKey::F12),
      Named::F13 => Some(HandyKey::F13),
      Named::F14 => Some(HandyKey::F14),
      Named::F15 => Some(HandyKey::F15),
      Named::F16 => Some(HandyKey::F16),
      Named::F17 => Some(HandyKey::F17),
      Named::F18 => Some(HandyKey::F18),
      Named::F19 => Some(HandyKey::F19),
      Named::F20 => Some(HandyKey::F20),
      _ => None,
    },
    IcedKey::Character(c) => match c.as_str() {
      "a" => Some(HandyKey::A),
      "b" => Some(HandyKey::B),
      "c" => Some(HandyKey::C),
      "d" => Some(HandyKey::D),
      "e" => Some(HandyKey::E),
      "f" => Some(HandyKey::F),
      "g" => Some(HandyKey::G),
      "h" => Some(HandyKey::H),
      "i" => Some(HandyKey::I),
      "j" => Some(HandyKey::J),
      "k" => Some(HandyKey::K),
      "l" => Some(HandyKey::L),
      "m" => Some(HandyKey::M),
      "n" => Some(HandyKey::N),
      "o" => Some(HandyKey::O),
      "p" => Some(HandyKey::P),
      "q" => Some(HandyKey::Q),
      "r" => Some(HandyKey::R),
      "s" => Some(HandyKey::S),
      "t" => Some(HandyKey::T),
      "u" => Some(HandyKey::U),
      "v" => Some(HandyKey::V),
      "w" => Some(HandyKey::W),
      "x" => Some(HandyKey::X),
      "y" => Some(HandyKey::Y),
      "z" => Some(HandyKey::Z),
      _ => None,
    },
  }
}

pub fn translate_event_to_hotkey(event: Event) -> Result<Option<Hotkey>> {
  match event {
    Event::KeyPressed {
      key,
      modified_key: _,
      physical_key: _,
      location: _,
      modifiers,
      text: _,
      repeat: _,
    } => {
      let key = translate_key_to_key(key);
      let mut new_modifiers = HandyModifiers::empty();
      if modifiers.contains(IcedModifiers::SHIFT) {
        new_modifiers.insert(HandyModifiers::SHIFT);
      }
      if modifiers.contains(IcedModifiers::CTRL) {
        new_modifiers.insert(HandyModifiers::CTRL);
      }
      if modifiers.contains(IcedModifiers::ALT) {
        new_modifiers.insert(HandyModifiers::OPT);
      }
      if modifiers.contains(IcedModifiers::LOGO) {
        new_modifiers.insert(HandyModifiers::CMD);
      }

      Ok(Hotkey::new(new_modifiers, key).map(Some).unwrap_or(None))
    }
    Event::KeyReleased {
      key,
      modified_key: _,
      physical_key: _,
      location: _,
      modifiers,
    } => {
      let key = translate_key_to_key(key);
      let mut new_modifiers = HandyModifiers::empty();
      if modifiers.contains(IcedModifiers::SHIFT) {
        new_modifiers.insert(HandyModifiers::SHIFT);
      }
      if modifiers.contains(IcedModifiers::CTRL) {
        new_modifiers.insert(HandyModifiers::CTRL);
      }
      if modifiers.contains(IcedModifiers::ALT) {
        new_modifiers.insert(HandyModifiers::OPT);
      }
      if modifiers.contains(IcedModifiers::LOGO) {
        new_modifiers.insert(HandyModifiers::CMD);
      }

      Ok(Hotkey::new(new_modifiers, key).map(Some).unwrap_or(None))
    }
    Event::ModifiersChanged(_) => Ok(None),
  }
}
