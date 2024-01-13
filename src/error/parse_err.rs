use crate::{lexer::Token, utils::StrShortHand};

use super::{CowErrorKind, ErrorConfig, ErrorViewConfig};

impl ErrorConfig {
  pub fn parse_unexpected_token<S: AsRef<str>>(&self, _expected: S, tkn: Token) -> ErrorViewConfig {
    let pre = self.text.get(0..45).unwrap_or_default().to_string();

    return ErrorViewConfig {
      row_index: 1,
      col_index: tkn.tkn_pos,
      err_kind: CowErrorKind::PARSER,
      err_msg: "unknown error occured".to_string(),
      input_pre: pre.clone(),
      err_marker: format!(" {}", "~".multiply(pre.chars().count() as i32)),
      help_msg: None,
    };
  }

  pub fn parse_big_int(&self, tkn_pos: usize, size: usize) -> ErrorViewConfig {
    let pre = self.split_preview(tkn_pos, tkn_pos + size);

    return ErrorViewConfig {
      row_index: 1,
      col_index: tkn_pos as i32,
      err_kind: CowErrorKind::PARSER,
      err_msg: format!("unable to parse a very big number"),
      input_pre: format!("{}{}{}", pre.0, pre.1, pre.2),
      err_marker: format!(
        "{}{}",
        " ".multiply(pre.0.chars().count() as i32),
        "~".multiply(pre.1.chars().count() as i32)
      ),
      help_msg: None,
    };
  }
}
