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
  let code = "1 + 2 * 3 % 4 / #5 - 6";
  // let code = "1 + 1 - 2 * 3 / 2 - 6 + 2 + 1 % 9 + 7 * 8 % 6";
  // let code = "2 + 5 - 5 * PI - (12 + 4) - 0b101010 + 0xffffff";

  let err_config = ErrorConfig::new(code.to_string(), 15);
  let lx = lexer::Tokenizer::new(code.to_string(), err_config);

  match lx.tokenize() {
    Ok(tkns) => {
      let proper = proper::Proper::new();
      let tokens = tkns.iter().map(|e| e).collect();
      println!("PASS A");

      match proper.proper_literals(tokens, &lx) {
        Ok(p) => {
          let parser = Parser::new();
          println!("PASS B");

          match parser.parse(p, &lx) {
            Ok(ast) => {
              let eval = Eval::new();
              println!("PASS C");

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
