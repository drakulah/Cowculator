use serde::Serialize;

mod fn_call;
mod proper;
mod scope;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum OpPriority {
  Low,
  Medium,
  High,
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ProperScope {
  ProperFloatLiteral(ProperFloatLiteral),
  ProperIntLiteral(ProperIntLiteral),
  ProperConstantLiteral(ProperConstantLiteral),
  ProperScopeList(ProperScopeList),
  ProperOpLiteral(ProperOpLiteral),
  ProperFnCall(ProperFnCall),
}

#[derive(Debug, Clone, Serialize)]
pub struct ProperScopeList {
  pub value: Vec<ProperScope>,
  pub inline_fn: Option<ProperFnCall>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProperConstantLiteral {
  pub value: String,
  pub inline_fn: Option<ProperFnCall>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProperIntLiteral {
  pub value: i64,
  pub inline_fn: Option<ProperFnCall>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProperFloatLiteral {
  pub value: f64,
  pub inline_fn: Option<ProperFnCall>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProperOpLiteral {
  pub value: String,
  pub priority: OpPriority,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProperFnCall {
  pub name: String,
  pub params: Vec<ProperScopeList>,
  pub inline_fn: Option<Box<ProperFnCall>>,
}

#[derive(Debug)]
pub struct Proper {}
