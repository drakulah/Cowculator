use serde::Serialize;

use crate::proper::{ProperOpLiteral, ProperScope};

mod parse;

#[derive(Debug)]
pub enum Number {
  I64(i64),
  F64(f64),
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Branch {
  ProperScope(ProperScope),
  Tree(Tree),
}

#[derive(Debug, Serialize)]
pub struct Tree {
  pub op: Option<ProperOpLiteral>,
  pub left: Option<Box<Branch>>,
  pub right: Option<Box<Branch>>,
}

#[derive(Debug)]
pub struct Parser {}
