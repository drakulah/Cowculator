use crate::error::ErrorViewConfig;

use super::{OpLiteral, Parser, Tree};

impl Parser {
  pub fn parse(&self) -> Result<Tree, ErrorViewConfig> {
    let mut ast = Tree {
      op: OpLiteral {
        value: String::from("+"),
      },
      left: None,
      right: None,
    };

    return Ok(ast);
  }
}
