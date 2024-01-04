use crate::error::ErrorConfig;

mod tokenize;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
  Int,
  Float,

  Id,

  LParen,
  RParen,

  Mod,
  Star,
  Slash,
  Plus,
  Minus,

  Comma,
  Dot,
}
#[derive(Debug)]
pub struct Token {
  pub tkn_type: TokenType,
  pub lexeme: String,
  pub tkn_pos: i32,
  pub tkn_len: i32,
}

impl Token {
  pub fn new<S: AsRef<str>>(token_type: TokenType, lexeme: S, tkn_pos: i32, tkn_len: i32) -> Token {
    Token {
      tkn_type: token_type,
      lexeme: lexeme.as_ref().to_string(),
      tkn_pos,
      tkn_len
    }
  }

  pub fn to_string(&self) -> String {
    format!(
      "    token_type: '{:?}',\n    lexeme: '{}'\n    token_pos: '{}'\n_________________________\n",
      self.tkn_type, self.lexeme, self.tkn_pos
    )
  }
}
#[derive(Debug)]
pub struct Tokenizer {
  text: String,
  pub err_config: ErrorConfig
}

impl Tokenizer {
  pub fn new(text: String, err_config: ErrorConfig) -> Tokenizer {
    Tokenizer { text, err_config }
  }
}
