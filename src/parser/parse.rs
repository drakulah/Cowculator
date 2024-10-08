use crate::{
  error::ErrorViewConfig,
  lexer::Tokenizer,
  proper::{OpPriority, ProperIntLiteral, ProperOpLiteral, ProperScope},
};

use super::{Branch, Parser, Tree};

impl Parser {
  pub fn new() -> Parser {
    Parser {}
  }

  pub fn parse(
    &self,
    scope: Vec<ProperScope>,
    tokenizer: &Tokenizer,
  ) -> Result<Option<Tree>, ErrorViewConfig> {
    let mut i = 0;

    if scope.len() == 1 {
      return Ok(Some(Tree {
        op: Some(ProperOpLiteral {
          value: "+".to_string(),
          priority: OpPriority::Low,
        }),
        left: Some(Box::new(Branch::ProperScope(scope[i].clone()))),
        right: Some(Box::new(Branch::ProperScope(
          ProperScope::ProperIntLiteral(ProperIntLiteral {
            value: 0,
            inline_fn: None,
          }),
        ))),
      }));
    } else if scope.len() > 2 {
      return match self.parse_low_priority_op(&mut i, &scope, tokenizer) {
        Ok(e) => Ok(Some(e)),
        Err(e) => Err(e),
      };
    } else {
      return Ok(None);
    }
  }

  fn parse_high_priority_op(
    &self,
    i: &mut usize,
    scope: &Vec<ProperScope>,
    tokenizer: &Tokenizer,
  ) -> Result<Tree, ErrorViewConfig> {
    let mut ast = Tree {
      op: None,
      left: None,
      right: None,
    };

    match scope[*i] {
      ProperScope::ProperOpLiteral(_) => {
        return Err(tokenizer.err_config.lex_unknown_err(0));
      }
      _ => {
        ast.left = Some(Box::new(Branch::ProperScope(
          scope.get(*i).unwrap().clone(),
        )))
      }
    }

    *i += 1;

    match &scope[*i] {
      ProperScope::ProperOpLiteral(it) => ast.op = Some(it.clone()),
      _ => {
        return Err(tokenizer.err_config.lex_unknown_err(0));
      }
    }

    *i += 1;

    while *i < scope.len() {
      let token = &scope[*i];

      match token {
        ProperScope::ProperOpLiteral(it) => {
          if it.priority != OpPriority::High {
            break;
          }

          if ast.op.is_some() && ast.left.is_some() && ast.right.is_some() {
            ast = Tree {
              op: Some(it.clone()),
              left: Some(Box::new(Branch::Tree(ast))),
              right: None,
            }
          } else {
            return Err(tokenizer.err_config.lex_unknown_err(0));
          }
        }
        _ => {
          if ast.right.is_none() {
            ast.right = Some(Box::new(Branch::ProperScope(token.clone())));
          } else {
            return Err(tokenizer.err_config.lex_unknown_err(0));
          }
        }
      }

      *i += 1;
    }

    if ast.left.is_none() || ast.right.is_none() || ast.op.is_none() {
      return Err(tokenizer.err_config.lex_unknown_err(0));
    }

    Ok(ast)
  }

  fn parse_medium_priority_op(
    &self,
    i: &mut usize,
    scope: &Vec<ProperScope>,
    tokenizer: &Tokenizer,
  ) -> Result<Tree, ErrorViewConfig> {
    let mut ast = Tree {
      op: None,
      left: None,
      right: None,
    };

    match scope[*i] {
      ProperScope::ProperOpLiteral(_) => {
        return Err(tokenizer.err_config.lex_unknown_err(0));
      }
      _ => {
        ast.left = Some(Box::new(Branch::ProperScope(
          scope.get(*i).unwrap().clone(),
        )))
      }
    }

    *i += 1;

    match &scope[*i] {
      ProperScope::ProperOpLiteral(it) => ast.op = Some(it.clone()),
      _ => {
        return Err(tokenizer.err_config.lex_unknown_err(0));
      }
    }

    *i += 1;

    while *i < scope.len() {
      let token = &scope[*i];

      match token {
        ProperScope::ProperOpLiteral(it) => {
          if it.priority == OpPriority::Low {
            break;
          }

          if ast.op.is_some() && ast.left.is_some() && ast.right.is_some() {
            ast = Tree {
              op: Some(it.clone()),
              left: Some(Box::new(Branch::Tree(ast))),
              right: None,
            }
          } else {
            return Err(tokenizer.err_config.lex_unknown_err(0));
          }
        }
        _ => {
          if ast.right.is_none() {
            let next_op_index = *i + 1;
            if next_op_index < scope.len() && scope[next_op_index].is_high_priority_op() {
              match self.parse_high_priority_op(i, scope, tokenizer) {
                Ok(parsed) => ast.right = Some(Box::new(Branch::Tree(parsed))),
                Err(e) => {
                  return Err(e);
                }
              }
              continue;
            } else {
              ast.right = Some(Box::new(Branch::ProperScope(token.clone())));
            }
          } else {
            return Err(tokenizer.err_config.lex_unknown_err(0));
          }
        }
      }

      *i += 1;
    }

    if ast.left.is_none() || ast.right.is_none() || ast.op.is_none() {
      return Err(tokenizer.err_config.lex_unknown_err(0));
    }

    Ok(ast)
  }

  fn parse_low_priority_op(
    &self,
    i: &mut usize,
    scope: &Vec<ProperScope>,
    tokenizer: &Tokenizer,
  ) -> Result<Tree, ErrorViewConfig> {
    let mut ast = Tree {
      op: None,
      left: None,
      right: None,
    };

    match scope[*i] {
      ProperScope::ProperOpLiteral(_) => {
        return Err(tokenizer.err_config.lex_unknown_err(0));
      }
      _ => {
        ast.left = Some(Box::new(Branch::ProperScope(
          scope.get(*i).unwrap().clone(),
        )))
      }
    }

    *i += 1;

    match &scope[*i] {
      ProperScope::ProperOpLiteral(it) => ast.op = Some(it.clone()),
      _ => {
        return Err(tokenizer.err_config.lex_unknown_err(0));
      }
    }

    *i += 1;

    while *i < scope.len() {
      let token = &scope[*i];

      match token {
        ProperScope::ProperOpLiteral(it) => {
          if ast.op.is_some() && ast.left.is_some() && ast.right.is_some() {
            ast = Tree {
              op: Some(it.clone()),
              left: Some(Box::new(Branch::Tree(ast))),
              right: None,
            }
          } else {
            return Err(tokenizer.err_config.lex_unknown_err(0));
          }
        }
        _ => {
          if ast.right.is_none() {
            let next_op_index = *i + 1;

            if next_op_index < scope.len() && !scope[next_op_index].is_low_priority_op() {
              match self.parse_medium_priority_op(i, scope, tokenizer) {
                Ok(parsed) => ast.right = Some(Box::new(Branch::Tree(parsed))),
                Err(e) => return Err(e),
              }
              continue;
            }

            ast.right = Some(Box::new(Branch::ProperScope(token.clone())));
          } else {
            return Err(tokenizer.err_config.lex_unknown_err(0));
          }
        }
      }

      *i += 1;
    }

    if ast.left.is_none() || ast.right.is_none() || ast.op.is_none() {
      return Err(tokenizer.err_config.lex_unknown_err(0));
    }

    Ok(ast)
  }
}
