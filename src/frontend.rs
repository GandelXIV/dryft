use std::collections::HashMap;
use crate::backends::Backend;
use crate::backends::MockBackend;
use crate::backends::c99::C99Backend;


#[derive(Debug, PartialEq)]
enum DefinitionTypes {
	Function,
	Action,
	Loop,
	Conditional,
}

pub struct CompileState {
	pub out: Option<String>, // access after compile() has been called
	pub log_tokens: Vec<String>, // purely for debugging usecases

	pub functions: HashMap<String, String>,
	pub actions: HashMap<String, String>,

	pub word: String,

	pub defnstack: Vec<DefinitionTypes>,
	pub metastack: Vec<Vec<String>>,
	pub bodystack: Vec<String>,
}


impl CompileState {
	fn new() -> Self {
		Self {
			out: None,
			log_tokens: vec![],
			word: String::new(),
			functions: HashMap::new(),
			actions: HashMap::new(),
			defnstack: vec![],
			metastack: vec![],
			bodystack: vec![String::new()],
		}
	}
}

pub fn compile<B: Backend>(mut backend: B, code: &str) -> CompileState {
	let mut cs = CompileState::new();

	macro_rules! new_token {
	    () => {{
	        if !cs.word.is_empty() {
	        	cs.log_tokens.push(cs.word.clone());
				handle_token(&mut backend, &mut cs);
				cs.word = String::new();
	        }
	    }};
	}

	for letter in code.chars() {
		match letter {
			' ' | '\n' | '\t' => new_token!(),
			other => cs.word.push(other),
		}
	}
	new_token!(); // last word may not be whitespace separated
	cs.out = Some(cs.bodystack.remove(0));

	return cs
}

fn handle_token<B: Backend>(backend: &mut B, cs: &mut CompileState) {
	macro_rules! new_definition {
	    ($variant:ident) => {{
	        cs.defnstack.push(DefinitionTypes::$variant);
			cs.metastack.push(vec![]);
			cs.bodystack.push(String::new());
	    }};
	}

	macro_rules! add2body {
		($generated:expr) => {{
			cs.bodystack.last_mut().unwrap().push_str($generated)
		}};
	}

	match cs.word.as_ref() {
		"fun:" => new_definition!(Function),

		";fun" => {
			if cs.defnstack.pop().unwrap() != DefinitionTypes::Function {
				panic!("DRYFTERR - Misplaced function block ending");
			}
			let meta = cs.metastack.pop().unwrap();
			let body = cs.bodystack.pop().unwrap();

			let fname = meta.get(0).expect("DRYFTERR - No function name provided");
			let f = backend.create_function(fname.as_ref(), body);
			add2body!(&f);
		}

		fname if *cs.defnstack.last().unwrap() == DefinitionTypes::Function && cs.metastack.last().unwrap().is_empty() => 
			cs.metastack.last_mut().unwrap().push(fname.into()),

		"+" => add2body!(backend.fun_add()),
		/*";fun" => {
			if cs.defstack.last().0 != DefinitionTypes::Function {
				panic!("DRYFTERR - Invalid endind inside function");
			}
			let mut block = cs.defstack.pop().1;
			let fname = block.remove(0);
			let f = backend.create_function()
		}*/
		word => add2body!(word),
	}
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
		let mut cs = compile(backend, "fun: inc\n\t1 + ;fun");
		assert_eq!(cs.log_tokens, makeStrings(vec!["fun:", "inc", "1", "+", ";fun"]));
		//assert_eq!(cs.defstack, vec![(DefinitionTypes::Function, makeStrings(vec!["inc", "1", "+", ";"]))]);
	}

	#[test]
	fn function_compilation() {
		let mut backend = C99Backend {};
		let mut cs = compile(backend, "fun: sum3 + + ;fun");

		assert_eq!( cs.out.unwrap(), "void fun_sum3() { add(); add(); }".to_string() );
	}
}