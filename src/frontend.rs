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

	pub iscomment: bool,
	pub isstring: bool,
	pub newstring: String,
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
			iscomment: false,
			isstring: false,
			newstring: String::new(),
		}
	}

	fn add2body(&mut self, s: &str) {
		self.bodystack.last_mut().unwrap().push_str(s)
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
			c if cs.iscomment => {
				if c == '#' {
					cs.iscomment = false;
				}
			}
			c if cs.isstring => {
				if c == '"' {
					cs.isstring = false;
					cs.add2body(&backend.push_string(&cs.newstring));
					cs.newstring = String::new();
				} else {
					cs.newstring.push(c);
				}
			}
			' ' | '\n' | '\t' => new_token!(),
			'#' => cs.iscomment = true,
			'"' => cs.isstring = true,
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

	macro_rules! add_function {
		() => {
			let meta = cs.metastack.pop().unwrap();
			let body = cs.bodystack.pop().unwrap();

			let fname = meta.get(0).expect("DRYFTERR - No function name provided");

			cs.functions.insert(fname.clone(), body.clone());

			let f = backend.create_function(fname.as_ref(), body);
			cs.add2body(&f);
		}
	}

	match cs.word.as_ref() {
		"fun:" | "fun" => new_definition!(Function),

		";fun" | "endfun" | ":fun" => {
			if cs.defnstack.pop().unwrap() != DefinitionTypes::Function {
				panic!("DRYFTERR - Misplaced function block ending");
			}
			add_function!();
		}

		";" | "end" => {
			match cs.defnstack.pop().expect("DRYFTERR - Misplaced ;") {
				DefinitionTypes::Function => { add_function!(); }
				_ => todo!(),
			}
		}

		mac if (false) => {} // future for macro expansion

		fname if *cs.defnstack.last().unwrap() == DefinitionTypes::Function && cs.metastack.last().unwrap().is_empty() => 
			cs.metastack.last_mut().unwrap().push(fname.into()),

		/// actual code must start here, as to not be confused for function name

		fun if cs.functions.contains_key(fun) => {
			cs.add2body(&backend.user_function(fun));
		}

		num if regexint.is_match(num) => cs.add2body(&backend.push_integer(num)),

		"+" => cs.add2body(backend.fun_add()),
		"-" => cs.add2body(backend.fun_sub()),
		"*" => cs.add2body(backend.fun_mul()),
		"/" => cs.add2body(backend.fun_div()),
		"mod" => cs.add2body(backend.fun_mod()),
		"^" => cs.add2body(backend.fun_copy()),
		"v" => cs.add2body(backend.fun_drop()),
		"puti" => cs.add2body(backend.act_print_integer()),
		"puts" => cs.add2body(backend.act_print_string()),
		
		word => panic!("DRYFTERR - Unknown token '{}'", word),
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

		assert_eq!( cs.out.unwrap(), "void fun_sum3() { add(); add(); }\n".to_string() );
	}

	#[test]
	fn strings() {
		let mut backend = C99Backend {};
		let mut cs = compile(backend, "fun idk \" # fake comment # \" ; ");
	}
}