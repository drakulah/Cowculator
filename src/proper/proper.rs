use crate::lexer::Tokenizer;
use crate::{
  error::ErrorViewConfig,
  lexer::{Token, TokenType},
};

use super::{
  OpPriority, Proper, ProperConstantLiteral, ProperFloatLiteral, ProperFnCall, ProperIntLiteral,
  ProperOpLiteral, ProperScope, ProperScopeList,
};

impl Proper {
  pub fn new() -> Proper {
    Proper {}
  }

  fn proper_scope<'a>(
    &'a self,
    i: &mut usize,
    tokens: &Vec<&'a Token>,
    tokenizer: &Tokenizer,
  ) -> Result<Vec<&Token>, ErrorViewConfig> {
    let mut lp_index = 0;
    let mut paren_count = 1;
    let mut proper_tkns = Vec::new();

    if let Some(token) = tokens.get(*i) {
      match token.tkn_type {
        TokenType::LParen => {
          lp_index = token.tkn_pos;
        }
        _ => {}
      }
    }

    *i += 1;

    while *i < tokens.len() {
      if let Some(token) = tokens.get(*i) {
        match token.tkn_type {
          TokenType::LParen => paren_count += 1,
          TokenType::RParen => {
            paren_count -= 1;
            if paren_count == 0 {
              return Ok(proper_tkns);
            }
          }
          _ => {}
        }

        proper_tkns.push(token);
      }

      *i += 1;
    }

    return Err(
      tokenizer
        .err_config
        .lex_expected_closing_paren(lp_index.try_into().unwrap()),
    );
  }

  fn break_lexical_fn_params<'a>(
    &'a self,
    i: &mut usize,
    tokens: &Vec<&'a Token>,
    tokenizer: &Tokenizer,
  ) -> Result<Vec<Vec<&Token>>, ErrorViewConfig> {
    let mut lp_index = 0;
    let mut paren_count = 1;
    let mut temp_tokens = Vec::new();
    let mut broken_token_groups: Vec<Vec<&Token>> = Vec::new();

    if let Some(token) = tokens.get(*i) {
      match token.tkn_type {
        TokenType::LParen => {
          lp_index = token.tkn_pos;
        }
        _ => {}
      }
    }

    *i += 1;

    while *i < tokens.len() {
      if let Some(token) = tokens.get(*i) {
        match token.tkn_type {
          TokenType::LParen => paren_count += 1,
          TokenType::RParen => {
            paren_count -= 1;

            if paren_count == 0 {
              broken_token_groups.push(temp_tokens);
              return Ok(broken_token_groups);
            }
          }
          TokenType::Comma => {
            if paren_count == 1 {
              broken_token_groups.push(temp_tokens);
              temp_tokens = Vec::new();
              *i += 1;
              continue;
            }
          }
          _ => {}
        }

        temp_tokens.push(token);
      }

      *i += 1;
    }

    return Err(
      tokenizer
        .err_config
        .lex_expected_closing_paren(lp_index.try_into().unwrap()),
    );
  }

  pub fn proper_literals(
    &self,
    tokens: Vec<&Token>,
    tokenizer: &Tokenizer,
  ) -> Result<Vec<ProperScope>, ErrorViewConfig> {
    let mut scope = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
      let tkn = tokens.get(i).unwrap();

      match tkn.tkn_type {
        TokenType::Int => {
          if let Ok(lexeme_i64) = tkn.lexeme.parse::<i64>() {
            if scope.len() == 1
              && scope
                .last()
                .is_some_and(|it: &ProperScope| it.is_minus_op())
            {
              scope.pop();
              scope.push(ProperScope::ProperIntLiteral(ProperIntLiteral {
                value: 0 - lexeme_i64,
                inline_fn: None,
              }));
            } else {
              if scope.len() == 1 && scope.last().is_some_and(|it: &ProperScope| it.is_plus_op()) {
                scope.pop();
              }
              scope.push(ProperScope::ProperIntLiteral(ProperIntLiteral {
                value: lexeme_i64,
                inline_fn: None,
              }));
            }
          } else {
            return Err(
              tokenizer
                .err_config
                .parse_big_int(tkn.tkn_pos as usize, tkn.lexeme.len()),
            );
          }
        }
        TokenType::Float => {
          if let Ok(lexeme_f64) = tkn.lexeme.parse::<f64>() {
            if scope.len() == 1
              && scope
                .last()
                .is_some_and(|it: &ProperScope| it.is_minus_op())
            {
              scope.pop();
              scope.push(ProperScope::ProperFloatLiteral(ProperFloatLiteral {
                value: 0_f64 - lexeme_f64,
                inline_fn: None,
              }));
            } else {
              if scope.len() == 1 && scope.last().is_some_and(|it: &ProperScope| it.is_plus_op()) {
                scope.pop();
              }
              scope.push(ProperScope::ProperFloatLiteral(ProperFloatLiteral {
                value: lexeme_f64,
                inline_fn: None,
              }));
            }
          } else {
            return Err(
              tokenizer
                .err_config
                .parse_big_int(tkn.tkn_pos as usize, tkn.lexeme.len()),
            );
          }
        }
        TokenType::Id => {
          scope.push(ProperScope::ProperConstantLiteral(ProperConstantLiteral {
            value: tkn.lexeme.clone(),
            inline_fn: None,
          }));
        }
        TokenType::LParen => {
          if let Some(last_tkn) = scope.last_mut() {
            match last_tkn {
              ProperScope::ProperConstantLiteral(prev_tkn) => {
                let mut fn_params = Vec::new();

                match self.break_lexical_fn_params(&mut i, &tokens, tokenizer) {
                  Ok(param_tkns) => {
                    for each_param in param_tkns.into_iter() {
                      match self.proper_literals(each_param, tokenizer) {
                        Ok(prop_literals) => fn_params.push(ProperScopeList {
                          value: prop_literals,
                          inline_fn: None,
                        }),
                        Err(e) => return Err(e),
                      }
                    }
                  }
                  Err(e) => return Err(e),
                }

                *last_tkn = ProperScope::ProperFnCall(ProperFnCall {
                  name: prev_tkn.value.to_string(),
                  params: fn_params,
                  inline_fn: None,
                });

                i += 1;
                continue;
              }
              _ => {}
            }
          }
          match self.proper_scope(&mut i, &tokens, tokenizer) {
            Ok(param_tkns) => match self.proper_literals(param_tkns, tokenizer) {
              Ok(prop_literals) => {
                if prop_literals.is_empty() {
                  // TODO: Throw `()` input error
                } else if prop_literals.len() == 1 {
                  scope.push(prop_literals.get(0).unwrap().clone());
                } else {
                  scope.push(ProperScope::ProperScopeList(ProperScopeList {
                    value: prop_literals,
                    inline_fn: None,
                  }))
                }
              }
              Err(e) => return Err(e),
            },
            Err(e) => return Err(e),
          }
        }
        TokenType::Mod => {
          scope.push(ProperScope::ProperOpLiteral(ProperOpLiteral {
            value: tkn.lexeme.clone(),
            priority: OpPriority::Medium,
          }));
        }
        TokenType::Star => {
          scope.push(ProperScope::ProperOpLiteral(ProperOpLiteral {
            value: tkn.lexeme.clone(),
            priority: OpPriority::High,
          }));
        }
        TokenType::Slash => {
          scope.push(ProperScope::ProperOpLiteral(ProperOpLiteral {
            value: tkn.lexeme.clone(),
            priority: OpPriority::High,
          }));
        }
        TokenType::Plus => {
          scope.push(ProperScope::ProperOpLiteral(ProperOpLiteral {
            value: tkn.lexeme.clone(),
            priority: OpPriority::Low,
          }));
        }
        TokenType::Minus => {
          scope.push(ProperScope::ProperOpLiteral(ProperOpLiteral {
            value: tkn.lexeme.clone(),
            priority: OpPriority::Low,
          }));
        }
        TokenType::Dot => {
          if i + 1 >= tokens.len() || tokens.is_empty() {
            return Err(
              tokenizer
                .err_config
                .lex_unexpected_token(tkn.tkn_pos as usize),
            );
          }

          i += 1;

          let next_tkn = tokens[i];

          if next_tkn.tkn_type != TokenType::Id {
            return Err(
              tokenizer
                .err_config
                .lex_unexpected_token(tkn.tkn_pos as usize),
            );
          }

          let mut inline_fn = ProperFnCall {
            name: next_tkn.lexeme.clone(),
            params: Vec::new(),
            inline_fn: None,
          };

          i += 1;

          match self.break_lexical_fn_params(&mut i, &tokens, tokenizer) {
            Ok(param_tkns) => {
              for each_param in param_tkns.into_iter() {
                match self.proper_literals(each_param, tokenizer) {
                  Ok(prop_literals) => inline_fn.params.push(ProperScopeList {
                    value: prop_literals,
                    inline_fn: None,
                  }),
                  Err(e) => return Err(e),
                }
              }
            }
            Err(e) => return Err(e),
          }

          if !scope
            .last_mut()
            .unwrap()
            .set_last_inline_fn(Some(inline_fn))
          {
            // TODO: handle inline_fn assigned to operator
          }
        }
        TokenType::RParen => {
          return Err(
            tokenizer
              .err_config
              .lex_unexpected_token(tkn.tkn_pos as usize),
          );
        }
        _ => {
          return Err(
            tokenizer
              .err_config
              .lex_unexpected_token(tkn.tkn_pos as usize),
          );
        }
      }

      i += 1;
    }

    return Ok(scope);
  }
}
