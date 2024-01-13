mod lex_err;
mod parse_err;
mod util;
mod view;

#[derive(Debug)]
pub enum CowErrorKind {
  LEXER,
  PARSER,
  EVALUATOR,
}

impl CowErrorKind {
  pub fn to_string(&self) -> String {
    match self {
      CowErrorKind::LEXER => String::from("lexer err"),
      CowErrorKind::PARSER => String::from("parse err"),
      CowErrorKind::EVALUATOR => String::from("eval err"),
    }
  }
}

#[derive(Debug)]
pub struct ErrorConfig {
  text: String,
  err_chunk: usize,
  err_pre_size: usize,
}

impl ErrorConfig {
  pub fn new(text: String, err_chunk: usize, err_pre_size: usize) -> ErrorConfig {
    ErrorConfig {
      text,
      err_chunk,
      err_pre_size,
    }
  }
}

#[derive(Debug)]
pub struct ErrorViewConfig {
  pub row_index: i32,
  pub col_index: i32,
  pub err_kind: CowErrorKind,
  pub err_msg: String,
  pub input_pre: String,
  pub err_marker: String,
  pub help_msg: Option<String>,
}

pub struct ErrorView {
  config: ErrorViewConfig,
}

impl ErrorView {
  pub fn new(config: ErrorViewConfig) -> ErrorView {
    ErrorView { config }
  }
}
