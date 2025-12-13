use crate::backends::Backend;
use crate::backends::MockBackend;

pub struct CompileState {
	pub out: String,
	pub log_tokens: Vec<String>,

	pub word: String,
}

impl CompileState {
	fn new() -> Self {
		Self {
			out: String::new(),
			log_tokens: vec![],
			word: String::new(),
		}
	}
}

pub fn compile<B: Backend>(mut backend: B, code: &str) -> CompileState {
	let mut cs = CompileState::new();

	macro_rules! new_token {
	    () => {{
	        if !cs.word.is_empty() {
	        	cs.log_tokens.push(cs.word.clone());
				let o = handle_token(&mut backend, &mut cs);
				cs.out.push_str(&o);
				cs.word = String::new();
	        }
	    }};
	}

	for letter in code.chars() {
		match letter {
			'\n' | '\t' => new_token!(),
			' ' => new_token!(),
			other => { cs.word.push(other) }
		}
	}
	new_token!(); // last word may not be whitespace separated

	return cs
}

fn handle_token<B: Backend>(backend: &mut B, cs: &mut CompileState) -> String {
	"".to_string()
}

fn makeStrings(v: Vec<&str>) -> Vec<String> {
	v.into_iter().map(String::from).collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn simple_parse() {
		let mut backend = MockBackend {};
		assert_eq!(compile(backend, "fun: inc\n\t1 + ;").log_tokens, makeStrings(vec!["fun:", "inc", "1", "+", ";"]));
	}
}