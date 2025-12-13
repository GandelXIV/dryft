use std::collections::HashMap;
use regex::Regex;
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

pub fn compile_full<B: Backend>(mut backend: B, code: &str) -> String {
	let built = compile(backend, code).out.expect("No code compiled :(");
	B::complete( &built )
}

fn handle_token<B: Backend>(backend: &mut B, cs: &mut CompileState) {
    let regexint = Regex::new(r"^-?\d+$").unwrap();

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

	macro_rules! add_function {
		() => {
			let meta = cs.metastack.pop().unwrap();
			let body = cs.bodystack.pop().unwrap();

			let fname = meta.get(0).expect("DRYFTERR - No function name provided");
			let f = backend.create_function(fname.as_ref(), body);
			add2body!(&f);
		}
	}

	match cs.word.as_ref() {
		"fun:" => new_definition!(Function),

		";fun" => {
			if cs.defnstack.pop().unwrap() != DefinitionTypes::Function {
				panic!("DRYFTERR - Misplaced function block ending");
			}
			add_function!();
		}

		";" => {
			match cs.defnstack.pop().expect("DRYFTERR - Misplaced ;") {
				DefinitionTypes::Function => { add_function!(); }
				_ => todo!(),
			}
		}

		fname if *cs.defnstack.last().unwrap() == DefinitionTypes::Function && cs.metastack.last().unwrap().is_empty() => 
			cs.metastack.last_mut().unwrap().push(fname.into()),

		/// actual code must start here, as to not be confused for function name

		num if regexint.is_match(num) => add2body!(&backend.push_integer(num)),

		"+" => add2body!(backend.fun_add()),
		"-" => add2body!(backend.fun_sub()),
		"*" => add2body!(backend.fun_mul()),
		"/" => add2body!(backend.fun_div()),
		"mod" => add2body!(backend.fun_mod()),
		"^" => add2body!(backend.fun_copy()),
		"v" => add2body!(backend.fun_drop()),
		"puti" => add2body!(backend.act_print_integer()),
		
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
	fn semicolon_ending() {
		let mut backend = MockBackend {};
		let mut cs = compile(backend, "fun: id ;");
		let mut backend = MockBackend {};
		let mut cs2 = compile(backend, "fun: id ;fun");
		assert_eq!(cs.out, cs2.out);
	}

	#[test]
	fn function_compilation() {
		let mut backend = C99Backend {};
		let mut cs = compile(backend, "fun: sum3 + + ;fun");

		assert_eq!( cs.out.unwrap(), "void fun_sum3() { add(); add(); }".to_string() );
	}
}