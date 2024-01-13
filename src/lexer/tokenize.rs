use crate::utils::is_alpha_numeric;
use crate::{error::ErrorViewConfig, utils::is_alpha};

use super::{Token, TokenType, Tokenizer};

impl Tokenizer {
  pub fn tokenize(&self) -> Result<Vec<Token>, ErrorViewConfig> {
    let mut tknz: Vec<Token> = Vec::new();
    let mut text_iter = self.text.chars().peekable();
    let mut iter = text_iter.next();
    let mut tkn_pos = 0;

    while iter.is_some() {
      let char = iter.unwrap();

      match char {
        '+' => tknz.push(Token::new(TokenType::Plus, "+", tkn_pos, 1)),
        '-' => tknz.push(Token::new(TokenType::Minus, "-", tkn_pos, 1)),
        '/' => tknz.push(Token::new(TokenType::Slash, "/", tkn_pos, 1)),
        '*' => tknz.push(Token::new(TokenType::Star, "*", tkn_pos, 1)),
        '%' => tknz.push(Token::new(TokenType::Mod, "%", tkn_pos, 1)),

        '(' => tknz.push(Token::new(TokenType::LParen, "(", tkn_pos, 1)),
        ')' => tknz.push(Token::new(TokenType::RParen, ")", tkn_pos, 1)),

        ',' => tknz.push(Token::new(TokenType::Comma, ",", tkn_pos, 1)),
        '.' => {
          let i_p = tkn_pos;
          let mut lxme = String::from(char);
          let mut maybe_char = text_iter.peek();
          if maybe_char.is_some_and(|e| !e.is_ascii_digit()) || maybe_char.is_none() {
            tknz.push(Token::new(TokenType::Dot, ".", tkn_pos, 1));
          } else {
            while maybe_char.is_some() {
              let next_char = maybe_char.unwrap();
              if next_char.is_ascii_digit()
                || ((next_char == &'e' || next_char == &'E')
                  && !lxme.contains('e')
                  && !lxme.ends_with('.'))
              {
                lxme.push(next_char.clone());
                text_iter.next();
                maybe_char = text_iter.peek();
                tkn_pos += 1;
              } else {
                break;
              }
            }

            if lxme.ends_with('e') {
              return Err(self.err_config.lex_unexpected_token(tkn_pos as usize));
            }

            tknz.push(Token::new(TokenType::Float, lxme, i_p, tkn_pos - i_p));
          }
        }

        ' ' => {}

        _ => {
          if is_alpha(&char) {
            let i_p = tkn_pos;
            let mut lxme = String::from(char);
            let mut maybe_char = text_iter.peek();

            while maybe_char.is_some() {
              let string_char = maybe_char.unwrap();
              if !is_alpha_numeric(&string_char) {
                break;
              }
              lxme.push(string_char.clone());
              text_iter.next();
              maybe_char = text_iter.peek();
              tkn_pos += 1;
            }

            tknz.push(Token::new(TokenType::Id, lxme, i_p, tkn_pos - i_p));
          } else if char.is_ascii_digit() {
            let i_p = tkn_pos;
            let mut lxme = String::from(char);
            let mut maybe_char = text_iter.peek();
            let mut base: i8 = 10;
            while maybe_char.is_some() {
              let inner_char = maybe_char.unwrap();

              if (inner_char == &'b' || inner_char == &'B') && lxme == "0" {
                base = 2;
                lxme.push(inner_char.clone());
                text_iter.next();
                maybe_char = text_iter.peek();
                tkn_pos += 1;
              } else if (inner_char == &'o' || inner_char == &'O') && lxme == "0" {
                base = 8;
                lxme.push(inner_char.clone());
                text_iter.next();
                maybe_char = text_iter.peek();
                tkn_pos += 1;
              } else if (inner_char == &'x' || inner_char == &'X') && lxme == "0" {
                base = 16;
                lxme.push(inner_char.clone());
                text_iter.next();
                maybe_char = text_iter.peek();
                tkn_pos += 1;
              } else if (inner_char == &'.' && !lxme.contains('.') && base == 10)
                || (inner_char.is_ascii_hexdigit() && base == 16)
                || (inner_char.is_ascii_digit() && base == 8 && inner_char != &'9')
                || (inner_char.is_ascii_digit()
                  && base == 2
                  && (inner_char == &'0' || inner_char == &'1'))
                || (inner_char.is_ascii_digit() && base == 10)
                || (inner_char.is_ascii_digit()
                  || ((inner_char == &'e' || inner_char == &'E')
                    && !lxme.contains('e')
                    && !lxme.ends_with('.')))
              {
                lxme.push(inner_char.clone());
                text_iter.next();
                maybe_char = text_iter.peek();
                tkn_pos += 1;
              } else {
                break;
              }
            }

            if lxme.ends_with('.') && text_iter.peek().is_none() || lxme.ends_with('e') {
              return Err(self.err_config.lex_unexpected_token(tkn_pos as usize));
            }

            let lxme_end_w_dot = lxme.ends_with('.');
            if lxme_end_w_dot {
              lxme.pop();
            }

            if lxme.contains('.') {
              tknz.push(Token::new(TokenType::Float, lxme, i_p, tkn_pos - i_p));
            } else {
              tknz.push(Token::new(TokenType::Int, lxme, i_p, tkn_pos - i_p));
            }

            if lxme_end_w_dot {
              tknz.push(Token::new(TokenType::Dot, ".", tkn_pos, 1));
            }
          } else {
            return Err(self.err_config.lex_err_unknown_token(tkn_pos as usize, 1));
          }
        }
      }

      tkn_pos += 1;
      iter = text_iter.next();
    }

    return Ok(tknz);
  }
}
