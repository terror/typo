use super::*;

#[derive(Debug, PartialEq)]
pub(crate) enum Action {
  Delete,
  Escape,
  Insert(char),
}

impl Action {
  pub(crate) fn from_event(event: Event) -> Option<Self> {
    match event {
      Event::Key(key) => match key.code {
        KeyCode::Backspace => Some(Self::Delete),
        KeyCode::Char(c) => Some(Self::Insert(c)),
        KeyCode::Esc => Some(Self::Escape),
        _ => None,
      },
      _ => None,
    }
  }
}

#[cfg(test)]
mod tests {
  use {
    super::*,
    crossterm::event::{KeyEvent, KeyModifiers},
  };

  #[test]
  fn from_event_backspace() {
    assert_eq!(
      Action::from_event(Event::Key(KeyEvent {
        code: KeyCode::Backspace,
        modifiers: KeyModifiers::NONE,
        kind: crossterm::event::KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
      })),
      Some(Action::Delete)
    );
  }

  #[test]
  fn from_event_char() {
    assert_eq!(
      Action::from_event(Event::Key(KeyEvent {
        code: KeyCode::Char('a'),
        modifiers: KeyModifiers::NONE,
        kind: crossterm::event::KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
      })),
      Some(Action::Insert('a'))
    );
  }

  #[test]
  fn from_event_escape() {
    assert_eq!(
      Action::from_event(Event::Key(KeyEvent {
        code: KeyCode::Esc,
        modifiers: KeyModifiers::NONE,
        kind: crossterm::event::KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
      })),
      Some(Action::Escape)
    );
  }

  #[test]
  fn from_event_unsupported_key() {
    assert_eq!(
      Action::from_event(Event::Key(KeyEvent {
        code: KeyCode::Enter,
        modifiers: KeyModifiers::NONE,
        kind: crossterm::event::KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
      })),
      None
    );
  }

  #[test]
  fn from_event_non_key_event() {
    assert_eq!(
      Action::from_event(Event::Mouse(crossterm::event::MouseEvent {
        kind: crossterm::event::MouseEventKind::Down(crossterm::event::MouseButton::Left),
        column: 0,
        row: 0,
        modifiers: KeyModifiers::NONE,
      })),
      None
    );
  }
}
