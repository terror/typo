use {
  crate::{action::Action, app::App, arguments::Arguments, state::State, statistics::Statistics},
  anyhow::{anyhow, bail},
  clap::Parser,
  crossterm::{
    cursor::{MoveTo, MoveToColumn},
    event::{self, Event, KeyCode},
    execute,
    style::{Color, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
  },
  rand::seq::SliceRandom,
  std::{
    cmp::Ordering,
    fmt::{self, Display, Formatter},
    io::{Write, stdout},
    process,
    time::{Duration, Instant},
  },
};

mod action;
mod app;
mod arguments;
mod state;
mod statistics;

const WORDS: &[&str] = &[
  "the", "be", "to", "of", "and", "a", "in", "that", "have", "i", "it", "for", "not", "on", "with",
  "he", "as", "you", "do", "at", "this", "but", "his", "by", "from", "they", "we", "say", "her",
  "she", "or", "an", "will", "my", "one", "all", "would", "there", "their", "what", "so", "up",
  "out", "if", "about", "who", "get", "which", "go", "me", "when", "make", "can", "like", "time",
  "no", "just", "him", "know", "take", "people", "into", "year", "your", "good", "some", "could",
  "them", "see", "other", "than", "then", "now", "look", "only", "come", "its", "over", "think",
  "also", "back", "after", "use", "two", "how", "our", "work", "first", "well", "way", "even",
  "new", "want", "because", "any", "these", "give", "day", "most", "us",
];

#[macro_export]
macro_rules! command {
  ($($cmd:expr),+ $(,)?) => {
    { execute!(stdout(), $($cmd),+) }
  };
}

type Result<T = (), E = anyhow::Error> = std::result::Result<T, E>;

fn main() {
  if let Err(error) = Arguments::parse().run() {
    eprintln!("error: {error}");
    process::exit(1);
  }
}
