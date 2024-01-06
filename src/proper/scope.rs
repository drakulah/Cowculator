use super::{ProperFnCall, ProperScope, OpPriority};

impl ProperScope {
  pub fn is_operator(&self) -> bool {
    match self {
      ProperScope::ProperOpLiteral(_) => true,
      _ => false,
    }
  }
  pub fn is_high_priority_op(&self) -> bool {
    match self {
      ProperScope::ProperOpLiteral(it) => it.priority == OpPriority::High,
      _ => false,
    }
  }
  pub fn is_medium_priority_op(&self) -> bool {
    match self {
      ProperScope::ProperOpLiteral(it) => it.priority == OpPriority::Medium,
      _ => false,
    }
  }
  pub fn is_low_priority_op(&self) -> bool {
    match self {
      ProperScope::ProperOpLiteral(it) => it.priority == OpPriority::Low,
      _ => false,
    }
  }
  pub fn is_minus_op(&self) -> bool {
    match self {
      ProperScope::ProperOpLiteral(it) => it.value == "-",
      _ => false,
    }
  }
  pub fn is_plus_op(&self) -> bool {
    match self {
      ProperScope::ProperOpLiteral(it) => it.value == "+",
      _ => false,
    }
  }
  pub fn has_inline_fn(&self) -> bool {
    match self {
      ProperScope::ProperFloatLiteral(it) => it.inline_fn.is_some(),
      ProperScope::ProperIntLiteral(it) => it.inline_fn.is_some(),
      ProperScope::ProperConstantLiteral(it) => it.inline_fn.is_some(),
      ProperScope::ProperScopeList(it) => it.inline_fn.is_some(),
      ProperScope::ProperFnCall(it) => it.inline_fn.is_some(),
      _ => false,
    }
  }
  pub fn set_inline_fn(&mut self, maybe_inline_fn: Option<ProperFnCall>) -> bool {
    match self {
      ProperScope::ProperFloatLiteral(it) => it.inline_fn = maybe_inline_fn,
      ProperScope::ProperIntLiteral(it) => it.inline_fn = maybe_inline_fn,
      ProperScope::ProperConstantLiteral(it) => it.inline_fn = maybe_inline_fn,
      ProperScope::ProperScopeList(it) => it.inline_fn = maybe_inline_fn,
      ProperScope::ProperFnCall(it) => {
        if let Some(inline_fn) = maybe_inline_fn {
          it.inline_fn = Some(Box::new(inline_fn));
        }
      }
      _ => {
        return false;
      }
    }

    return true;
  }
  pub fn set_last_inline_fn(&mut self, maybe_inline_fn: Option<ProperFnCall>) -> bool {
    match self {
      ProperScope::ProperFloatLiteral(it) => {
        if let Some(inline_fn_tkn) = &mut it.inline_fn {
          return inline_fn_tkn.set_last_inline_fn(maybe_inline_fn);
        } else {
          it.inline_fn = maybe_inline_fn;
        }
      }
      ProperScope::ProperIntLiteral(it) => {
        if let Some(inline_fn_tkn) = &mut it.inline_fn {
          return inline_fn_tkn.set_last_inline_fn(maybe_inline_fn);
        } else {
          it.inline_fn = maybe_inline_fn;
        }
      }
      ProperScope::ProperConstantLiteral(it) => {
        if let Some(inline_fn_tkn) = &mut it.inline_fn {
          return inline_fn_tkn.set_last_inline_fn(maybe_inline_fn);
        } else {
          it.inline_fn = maybe_inline_fn;
        }
      }
      ProperScope::ProperScopeList(it) => {
        if let Some(inline_fn_tkn) = &mut it.inline_fn {
          return inline_fn_tkn.set_last_inline_fn(maybe_inline_fn);
        } else {
          it.inline_fn = maybe_inline_fn;
        }
      }
      ProperScope::ProperFnCall(it) => {
        return it.set_last_inline_fn(maybe_inline_fn);
      }
      _ => {
        return false;
      }
    }

    return true;
  }
}
