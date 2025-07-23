use super::*;

#[derive(Debug, Parser)]
pub(crate) struct Arguments {
  #[clap(long, short, default_value = "100")]
  word_count: usize,
}

impl Arguments {
  pub(crate) fn run(self) -> Result {
    App::new(self.word_count).run()
  }
}
