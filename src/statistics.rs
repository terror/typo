use super::*;

#[derive(Debug, Clone)]
pub(crate) struct Statistics {
  pub(crate) accuracy: f64,
  pub(crate) elapsed_time: f64,
  pub(crate) errors: usize,
  pub(crate) wpm: f64,
}

impl Display for Statistics {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "WPM: {:.1} | Errors: {} | Accuracy: {:.1}% | Elapsed Time: {:.2}s",
      self.wpm, self.errors, self.accuracy, self.elapsed_time
    )
  }
}
