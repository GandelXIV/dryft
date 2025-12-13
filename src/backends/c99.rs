use crate::backends::Backend;

pub struct C99Backend {}

impl Backend for C99Backend {
	fn complete(compiled: &str) -> String {
		let mut cbase = include_str!("c99base.c").to_string();
		cbase.push_str(compiled);
		cbase
	}

	fn create_function(&self, fname: &str, body: String) -> String {
		format!("void fun_{}() {{ {}}}", fname, body)
	}

	fn fun_add(&self) -> &'static str {
		"add(); "
	}

	fn fun_sub(&self) -> &'static str {
		"sub(); "
	}

	fn fun_mul(&self) -> &'static str {
		"mul(); "
	}

	fn fun_div(&self) -> &'static str {
		"div(); "
	}

	fn fun_mod(&self) -> &'static str {
		"mod(); "
	}

	fn fun_copy(&self) -> &'static str {
		"copy(); "
	}

	fn fun_drop(&self) -> &'static str {
		"drop(); "
	}

	fn act_print_integer(&self) -> &'static str {
		"puti(); "
	}

	fn push_integer(&self, i: &str) -> String {
		format!("psh({i}); ")
	}
}