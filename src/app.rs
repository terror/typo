use super::*;

#[derive(Clone, Debug)]
pub(crate) struct App {
  characters: usize,
  errors: usize,
  input: String,
  position: usize,
  start_time: Instant,
  text: String,
}

impl Default for App {
  fn default() -> Self {
    Self {
      characters: 0,
      errors: 0,
      input: String::new(),
      position: 0,
      start_time: Instant::now(),
      text: String::new(),
    }
  }
}

impl App {
  pub(crate) fn new() -> Self {
    let mut generator = rand::thread_rng();

    let text = (0..100)
      .map(|_| WORDS.choose(&mut generator).unwrap())
      .cloned()
      .collect::<Vec<&str>>();

    Self {
      characters: 0,
      errors: 0,
      input: String::new(),
      position: 0,
      start_time: Instant::now(),
      text: text.join(" "),
    }
  }

  fn accuracy(&self) -> Result<f64> {
    if self.characters == 0 {
      return Ok(100.00);
    }

    let correct_chars = self
      .characters
      .checked_sub(self.errors)
      .ok_or_else(|| anyhow!("character count underflow"))?;

    let accuracy = (correct_chars as f64 / self.characters as f64) * 100.0;

    if accuracy.is_finite() {
      Ok(accuracy)
    } else {
      Err(anyhow!("accuracy calculation produced invalid result"))
    }
  }

  fn display(&self) -> Result {
    command!(Clear(ClearType::All), MoveTo(0, 0))?;

    let input_characters = self.input.chars().collect::<Vec<char>>();

    for (i, expected_character) in self.text.chars().enumerate() {
      command!(SetForegroundColor(match i.cmp(&self.position) {
        Ordering::Less => match input_characters.get(i) {
          Some(&typed_character) if typed_character == expected_character => Color::Green,
          _ => Color::Red,
        },
        Ordering::Equal => Color::Yellow,
        Ordering::Greater => Color::White,
      }))?;

      print!("{}", expected_character);
    }

    command!(ResetColor, MoveToColumn(0))?;

    print!("\n\n{}", self.statistics()?);

    stdout().flush()?;

    Ok(())
  }

  fn handle_action(&mut self, action: Action) -> State {
    match action {
      Action::Delete => {
        if !self.input.is_empty() {
          self.input.pop();

          if self.position > 0 {
            self.position -= 1;
          }
        }

        State::Continuing
      }
      Action::Escape => State::Quit,
      Action::Insert(c) => {
        let target_chars = self.text.chars().collect::<Vec<char>>();

        if self.position < target_chars.len() {
          let expected_char = target_chars[self.position];

          self.input.push(c);
          self.characters += 1;

          if c != expected_char {
            self.errors += 1;
          }

          self.position += 1;

          if self.position >= target_chars.len() {
            State::Completed
          } else {
            State::Continuing
          }
        } else {
          State::Continuing
        }
      }
    }
  }

  pub(crate) fn run(&mut self) -> Result {
    terminal::enable_raw_mode()?;

    loop {
      self.display()?;

      if event::poll(Duration::from_millis(100))? {
        if let Some(action) = Action::from_event(event::read()?) {
          match self.handle_action(action) {
            State::Completed => {
              command!(Clear(ClearType::All), MoveTo(0, 0))?;
              println!("{}\n", self.statistics()?);
              break;
            }
            State::Quit => break,
            State::Continuing => continue,
          }
        }
      }
    }

    terminal::disable_raw_mode()?;

    Ok(())
  }

  fn statistics(&self) -> Result<Statistics> {
    Ok(Statistics {
      accuracy: self.accuracy()?,
      elapsed_time: self.start_time.elapsed().as_secs_f64(),
      errors: self.errors,
      wpm: self.wpm()?,
    })
  }

  fn wpm(&self) -> Result<f64> {
    let elapsed = self.start_time.elapsed().as_secs_f64();

    if elapsed <= 0.0 {
      bail!("no time has elapsed since starting");
    }

    let words = self
      .position
      .checked_div(5)
      .ok_or_else(|| anyhow!("position value too large for calculation"))?;

    let minutes = elapsed / 60.0;

    if minutes == 0.0 {
      bail!("insufficient time elapsed to calculate wpm");
    }

    let wpm = (words as f64) / minutes;

    if wpm.is_finite() {
      Ok(wpm)
    } else {
      Err(anyhow!("wpm calculation produced invalid result"))
    }
  }
}

#[cfg(test)]
mod tests {
  use {super::*, approx::assert_abs_diff_eq};

  #[test]
  fn type_correct_char() {
    let mut app = App {
      text: "hello".into(),
      ..Default::default()
    };

    assert_eq!(app.handle_action(Action::Insert('h')), State::Continuing);

    assert_eq!(app.position, 1);
    assert_eq!(app.input, "h");
    assert_eq!(app.errors, 0);
    assert_eq!(app.characters, 1);
  }

  #[test]
  fn type_incorrect_char() {
    let mut app = App {
      text: "hello".into(),
      ..Default::default()
    };

    assert_eq!(app.handle_action(Action::Insert('x')), State::Continuing);

    assert_eq!(app.position, 1);
    assert_eq!(app.input, "x");
    assert_eq!(app.errors, 1);
    assert_eq!(app.characters, 1);
  }

  #[test]
  fn complete_typing() {
    let mut app = App {
      text: "hi".into(),
      ..Default::default()
    };

    assert_eq!(app.handle_action(Action::Insert('h')), State::Continuing);

    assert_eq!(app.handle_action(Action::Insert('i')), State::Completed);
  }

  #[test]
  fn backspace() {
    let mut app = App {
      text: "hello".into(),
      ..Default::default()
    };

    for c in "he".chars() {
      assert_eq!(app.handle_action(Action::Insert(c)), State::Continuing);
    }

    assert_eq!(app.input, "he");
    assert_eq!(app.position, 2);

    assert_eq!(app.handle_action(Action::Delete), State::Continuing);

    assert_eq!(app.input, "h");
    assert_eq!(app.position, 1);
  }

  #[test]
  fn backspace_empty() {
    let mut app = App {
      text: "hello".into(),
      ..Default::default()
    };

    assert_eq!(app.handle_action(Action::Delete), State::Continuing);

    assert_eq!(app.input, "");
    assert_eq!(app.position, 0);
  }

  #[test]
  fn accuracy() {
    let mut app = App {
      text: "test".into(),
      ..Default::default()
    };

    assert_eq!(app.accuracy().unwrap(), 100.0);

    assert_eq!(app.handle_action(Action::Insert('t')), State::Continuing);
    assert_eq!(app.accuracy().unwrap(), 100.0);

    assert_eq!(app.handle_action(Action::Insert('x')), State::Continuing);
    assert_eq!(app.accuracy().unwrap(), 50.0);

    assert_eq!(app.handle_action(Action::Insert('s')), State::Continuing);

    assert_abs_diff_eq!(app.accuracy().unwrap(), 66.66, epsilon = 0.01);

    assert_eq!(app.handle_action(Action::Insert('t')), State::Completed);

    assert_abs_diff_eq!(app.accuracy().unwrap(), 75.0, epsilon = 0.01);
  }

  #[test]
  fn wpm() {
    let mut app = App {
      text: "hello world test".into(),
      ..Default::default()
    };

    app.start_time = Instant::now() - Duration::from_secs(60);

    for c in "hello worl".chars() {
      app.handle_action(Action::Insert(c));
    }

    assert_abs_diff_eq!(app.wpm().unwrap(), 2.0, epsilon = 0.01);

    app.start_time = Instant::now() - Duration::from_secs(30);

    assert_abs_diff_eq!(app.wpm().unwrap(), 4.0, epsilon = 0.01);

    for c in "d test".chars() {
      app.handle_action(Action::Insert(c));
    }

    assert_abs_diff_eq!(app.wpm().unwrap(), 6.0, epsilon = 0.01);
  }
}
