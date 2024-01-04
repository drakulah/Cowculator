use crate::lexer::{Token, TokenType};

pub fn is_alpha(c: &char) -> bool {
  c.is_ascii_alphabetic() || c == &'_'
}

pub fn is_alpha_numeric(c: &char) -> bool {
  is_alpha(c) || c.is_ascii_digit()
}

pub fn is_num_tkn(tkn: &Token) -> bool {
  tkn.tkn_type == TokenType::Int || tkn.tkn_type == TokenType::Float
}

pub trait CoerceAtMost {
  fn coerce_at_most(self, max: Self) -> Self;
}

impl CoerceAtMost for usize {
  fn coerce_at_most(self, max: usize) -> usize {
    if self <= max {
      self
    } else {
      max
    }
  }
}

pub trait StrShortHand {
  fn multiply(&self, times: i32) -> String;
}

impl StrShortHand for &str {
  fn multiply(&self, t: i32) -> String {
    let mut v = String::new();

    for _i in 0..t {
      v.push_str(*self);
    }

    return v;
  }
}
