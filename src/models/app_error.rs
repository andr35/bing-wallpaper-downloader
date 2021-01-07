use std::error::Error;
use std::fmt::{Display, Formatter, Result};

/// Any generic error thrown by th application while running.
#[derive(Debug)]
pub struct AppError {
  details: String,
}

impl Display for AppError {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{}", self.details)
  }
}

impl AppError {
  #[allow(unused)]
  pub fn new(msg: &str) -> AppError {
    AppError {
      details: msg.to_string(),
    }
  }
}

impl Error for AppError {
  fn description(&self) -> &str {
    &self.details
  }
}
