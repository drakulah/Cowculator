use crate::{utils::StrShortHand, lexer::Token};

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

  pub fn parse_big_int(&self, s: &str) -> ErrorViewConfig {

    let pre = self.text.get(0..45).unwrap_or_default().to_string();

    return ErrorViewConfig {
      row_index: 1,
      col_index: 1,
      err_kind: CowErrorKind::PARSER,
      err_msg: s.to_string(),
      input_pre: pre.clone(),
      err_marker: format!(" {}", "~".multiply(pre.chars().count() as i32)),
      help_msg: None,
    };

  }

  pub fn lex_unknown_err(&self, tkn_pos: usize) -> ErrorViewConfig {
    let pre = self.text.get(0..45).unwrap_or_default().to_string();

    return ErrorViewConfig {
      row_index: 1,
      col_index: tkn_pos as i32,
      err_kind: CowErrorKind::LEXER,
      err_msg: "unknown error occured".to_string(),
      input_pre: pre.clone(),
      err_marker: format!(" {}", "~".multiply(pre.chars().count() as i32)),
      help_msg: None,
    };
  }

  pub fn lex_err_unknown_token(&self, tkn_pos: usize, tkn_len: usize) -> ErrorViewConfig {
    let lxme_end = tkn_pos + tkn_len;
    let err_chunk_1 = self.err_chunk - 1;
    let lxme;
    let f_pre;
    let b_pre;

    if (lxme_end - tkn_pos) <= self.err_chunk {
      if let Some(l) = self.text.get(tkn_pos..lxme_end) {
        lxme = l.to_string();
      } else {
        return self.lex_unknown_err(tkn_pos);
      }
    } else {
      if let Some(l) = self.text.get(tkn_pos..tkn_pos + err_chunk_1) {
        lxme = format!("{}…", l);
      } else {
        return self.lex_unknown_err(tkn_pos);
      }
    }

    if tkn_pos <= self.err_chunk {
      if let Some(l) = self.text.get(0..tkn_pos) {
        f_pre = l.to_string();
      } else {
        return self.lex_unknown_err(tkn_pos);
      }
    } else {
      if let Some(l) = self.text.get((tkn_pos - err_chunk_1)..tkn_pos) {
        f_pre = format!("…{}", l);
      } else {
        return self.lex_unknown_err(tkn_pos);
      }
    }

    if (lxme_end + self.err_chunk) >= self.text.len() {
      if let Some(l) = self.text.get(lxme_end..) {
        b_pre = l.to_string();
      } else {
        return self.lex_unknown_err(tkn_pos);
      }
    } else {
      if let Some(l) = self.text.get(lxme_end..(lxme_end + err_chunk_1)) {
        b_pre = format!("{}…", l);
      } else {
        return self.lex_unknown_err(tkn_pos);
      }
    }

    return ErrorViewConfig {
      row_index: 1,
      col_index: tkn_pos as i32,
      err_kind: CowErrorKind::LEXER,
      err_msg: format!("unknown token `{}`", lxme),
      input_pre: format!("{}{}{}", f_pre, lxme, b_pre),
      err_marker: format!(
        "{}{}",
        " ".multiply(f_pre.chars().count() as i32),
        "~".multiply(lxme.chars().count() as i32)
      ),
      help_msg: None,
    };
  }
}
