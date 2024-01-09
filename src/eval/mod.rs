use std::collections::HashMap;

mod evaluator;

pub struct Eval {
  constants: HashMap<String, f64>,
  inline_fn: HashMap<String, fn(Vec<f64>) -> f64>,
  usrdef_fn: HashMap<String, fn(Vec<f64>) -> f64>,
}
