use super::ProperFnCall;

impl ProperFnCall {
	pub fn set_last_inline_fn(&mut self, maybe_inline_fn: Option<ProperFnCall>) -> bool {
		if let Some(inline_fn_tkn) = &mut self.inline_fn {
			inline_fn_tkn.set_last_inline_fn(maybe_inline_fn);
		} else {
			if let Some(inline_fn) = maybe_inline_fn {
				self.inline_fn = Some(Box::new(inline_fn));
			} else {
				self.inline_fn = None;
			}
		}
		return true;
	}
}