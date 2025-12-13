use crate::backends::Backend;

pub struct C99Backend {}

impl Backend for C99Backend {
	fn complete(&self, compiled: &str) -> String {
		let mut cbase = include_str!("cbase.c").to_string();
		cbase.push_str(compiled);
		cbase
	}

	fn fun_add(&self) -> &'static str {
		"add(); "
	}

	fn create_function(&self, fname: &str, body: String) -> String {
		format!("void fun_{}() {{ {}}}", fname, body)
	}
}