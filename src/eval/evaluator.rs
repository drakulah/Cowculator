use std::collections::HashMap;

use crate::{
  error::ErrorViewConfig,
  lexer::Tokenizer,
  parser::{Branch, Parser, Tree},
  proper::{ProperFnCall, ProperScope},
};

use super::Eval;

impl Eval {
  pub fn new() -> Eval {
    Eval {
      constants: HashMap::new(),
      inline_fn: HashMap::new(),
      usrdef_fn: HashMap::new(),
    }
  }

  pub fn set_constant(&mut self, label: &str, value: f64) {
    self.constants.insert(label.to_string(), value);
  }

  pub fn set_inline_fn(&mut self, label: &str, value: fn(Vec<f64>) -> f64) {
    self.inline_fn.insert(label.to_string(), value);
  }

  pub fn compile_usedef_fn(
    &self,
    func: ProperFnCall,
    tokenizer: &Tokenizer,
    parser: &Parser,
  ) -> Result<f64, ErrorViewConfig> {
    if !self.usrdef_fn.contains_key(&func.name) {
      // TODO: Throw unknown inline function error
      return Err(tokenizer.err_config.lex_unknown_err(0));
    }

    let mut fn_params = Vec::new();
    let userdef_fn = self.usrdef_fn.get(&func.name).unwrap();

    for p in func.params.into_iter() {
      match parser.parse(p.value, tokenizer) {
        Ok(ast) => match self.compile(ast, tokenizer, parser) {
          Ok(value_b) => match self.compile_inline_fn(value_b, p.inline_fn, tokenizer, parser) {
            Ok(value_a) => fn_params.push(value_a),
            Err(e) => return Err(e),
          },
          Err(e) => return Err(e),
        },
        Err(e) => return Err(e),
      }
    }

    let inline_fn = match func.inline_fn {
      Some(v) => Some(*v),
      None => None,
    };

    return self.compile_inline_fn(userdef_fn(fn_params), inline_fn, tokenizer, parser);
  }

  pub fn compile_inline_fn(
    &self,
    value: f64,
    inline_fn: Option<ProperFnCall>,
    tokenizer: &Tokenizer,
    parser: &Parser,
  ) -> Result<f64, ErrorViewConfig> {
    match inline_fn {
      Some(v) => {
        let mut new_value = value;
        let mut it = Some(Box::from(v));

        while it.is_some() {
          let it_u = it.unwrap();

          if !self.inline_fn.contains_key(&it_u.name) {
            // TODO: Throw unknown inline function error
            return Err(tokenizer.err_config.lex_unknown_err(0));
          }

          let mut fn_params = vec![new_value];
          let userdef_fn = self.inline_fn.get(&it_u.name).unwrap();

          for p in it_u.params.into_iter() {
            match parser.parse(p.value, tokenizer) {
              Ok(ast) => match self.compile(ast, tokenizer, parser) {
                Ok(value_b) => {
                  match self.compile_inline_fn(value_b, p.inline_fn, tokenizer, parser) {
                    Ok(value_a) => fn_params.push(value_a),
                    Err(e) => return Err(e),
                  }
                }
                Err(e) => return Err(e),
              },
              Err(e) => return Err(e),
            }
          }

          new_value = userdef_fn(fn_params);
          it = it_u.inline_fn;
        }

        return Ok(new_value);
      }
      None => return Ok(value),
    };
  }

  pub fn compile_scope(
    &self,
    scope: ProperScope,
    tokenizer: &Tokenizer,
    parser: &Parser,
  ) -> Result<f64, ErrorViewConfig> {
    match scope {
      ProperScope::ProperFloatLiteral(value_f64) => {
        return self.compile_inline_fn(value_f64.value, value_f64.inline_fn, tokenizer, parser);
      }
      ProperScope::ProperIntLiteral(value_i64) => {
        return self.compile_inline_fn(
          value_i64.value as f64,
          value_i64.inline_fn,
          tokenizer,
          parser,
        );
      }
      ProperScope::ProperConstantLiteral(value_const) => {
        if let Some(value_f64) = self.constants.get(&value_const.value) {
          return self.compile_inline_fn(
            value_f64.clone(),
            value_const.inline_fn,
            tokenizer,
            parser,
          );
        } else {
          // TODO: Operator cannot be empty
          return Err(tokenizer.err_config.lex_unknown_err(0));
        }
      }
      ProperScope::ProperScopeList(value_scope) => {
        match parser.parse(value_scope.value, tokenizer) {
          Ok(ast) => match self.compile(ast, tokenizer, parser) {
            Ok(value_b) => {
              match self.compile_inline_fn(value_b, value_scope.inline_fn, tokenizer, parser) {
                Ok(value_a) => return Ok(value_a),
                Err(e) => return Err(e),
              }
            }
            Err(e) => return Err(e),
          },
          Err(e) => return Err(e),
        }
      }
      ProperScope::ProperFnCall(_) => todo!(),
      _ => {
        // TODO: Operator cannot be LHS | RHS
        return Err(tokenizer.err_config.lex_unknown_err(0));
      }
    };
  }

  pub fn compile(
    &self,
    tree: Option<Tree>,
    tokenizer: &Tokenizer,
    parser: &Parser,
  ) -> Result<f64, ErrorViewConfig> {
    if let Some(ast) = tree {
      let left: Option<f64>;
      let right: Option<f64>;

      match ast.left {
        Some(lhs) => match *lhs {
          Branch::ProperScope(scope) => match self.compile_scope(scope, tokenizer, parser) {
            Ok(res) => left = Some(res),
            Err(e) => return Err(e),
          },
          Branch::Tree(inner_tree) => match self.compile(Some(inner_tree), tokenizer, parser) {
            Ok(res) => left = Some(res),
            Err(e) => return Err(e),
          },
        },
        None => left = None,
      }

      if left.is_none() {
        // TODO: LHS cannot be empty
        return Err(tokenizer.err_config.lex_unknown_err(0));
      }

      match ast.right {
        Some(rhs) => match *rhs {
          Branch::ProperScope(scope) => match self.compile_scope(scope, tokenizer, parser) {
            Ok(res) => right = Some(res),
            Err(e) => return Err(e),
          },
          Branch::Tree(inner_tree) => match self.compile(Some(inner_tree), tokenizer, parser) {
            Ok(res) => right = Some(res),
            Err(e) => return Err(e),
          },
        },
        None => right = None,
      }

      if right.is_none() {
        // TODO: RHS cannot be empty
        return Err(tokenizer.err_config.lex_unknown_err(0));
      }

      if let Some(op) = ast.op {
        match op.value.as_str() {
          "*" => return Ok(left.unwrap() * right.unwrap()),
          "/" => return Ok(left.unwrap() / right.unwrap()),
          "%" => return Ok(left.unwrap() % right.unwrap()),
          "+" => return Ok(left.unwrap() + right.unwrap()),
          "-" => return Ok(left.unwrap() - right.unwrap()),
          _ => {}
        };
      }

      // TODO: Operator cannot be empty
      return Err(tokenizer.err_config.lex_unknown_err(0));
    } else {
      Ok(0.0)
    }
  }
}
