use error::ErrorConfig;
use crate::error::ErrorView;

pub mod color;
pub mod error;
pub mod lexer;
pub mod proper;
pub mod parser;
pub mod utils;

fn main() {
	let code = r#"2 + (-5) - (12 + 4).toString()"#;
	// let code = r#"2 + 5 - 5 * PI - (12 + 4) - 0b101010 + 0xffffff"#;

	let err_config = ErrorConfig::new(code.to_string(), 15);
	let lx = lexer::Tokenizer::new(code.to_string(), err_config);

	match lx.tokenize() {
		Ok(tkns) => {
			let proper = proper::Proper::new();
			let tokens = tkns.iter().map(|e| e).collect();

			match proper.proper_literals(tokens, &lx) {
				Ok(p) => {
					println!("{:#?}", p);
				}
				Err(e) => ErrorView::new(e).display()
			}
		}
		Err(e) => ErrorView::new(e).display()
	}
}
