use std::f64::consts::PI;

use crate::{error::ErrorView, eval::Eval, parser::Parser};
use error::ErrorConfig;

pub mod color;
pub mod error;
pub mod eval;
pub mod lexer;
pub mod parser;
pub mod proper;
pub mod utils;

fn main() {
  let code = "2 * 3 / PI.round() + 4";
  // let code = "1 + 1 - 2 * 3 / 2 - 6 + 2 + 1 % 9 + 7 * 8 % 6";
  // let code = "2 + 5 - 5 * PI - (12 + 4) - 0b101010 + 0xffffff";

  let err_config = ErrorConfig::new(code.to_string(), 15, 50);
  let lx = lexer::Tokenizer::new(code.to_string(), err_config);
  let mut eval = Eval::new();

  /* Declare compiler constants */
  eval.set_constant("PI", PI);

  /* Declare compiler inline functions */
  eval.set_inline_fn("round", |v| v[0].round());
  eval.set_inline_fn("floor", |v| v[0].floor());

  match lx.tokenize() {
    Ok(tkns) => {
      let proper = proper::Proper::new();
      let tokens = tkns.iter().map(|e| e).collect();
      // println!("PASS A");
      // println!("{:#?}", tkns);

      match proper.proper_literals(tokens, &lx) {
        Ok(p) => {
          let parser = Parser::new();
          // println!("PASS B");

          match parser.parse(p, &lx) {
            Ok(ast) => {
              // println!("PASS C");
              // println!("{:#?}", ast);

              match eval.compile(ast, &lx, &parser) {
                Ok(res) => println!("{}", res),
                Err(e) => ErrorView::new(e).display(),
              }
            }
            Err(e) => ErrorView::new(e).display(),
          }
        }
        Err(e) => ErrorView::new(e).display(),
      }
    }
    Err(e) => ErrorView::new(e).display(),
  }
}

// let json_str = serde_json::to_string(&ast);
// println!("{:?}", json_str.unwrap());
