pub mod c99;

pub trait Backend {
	fn complete( compiled: &str) -> String;

	fn create_function(&self, fname: &str, body: String) -> String;
	fn push_integer(&self, i: &str) -> String;
	fn user_function(&self, f: &str) -> String;

	fn fun_add(&self) -> &'static str;
	fn fun_sub(&self) -> &'static str;
	fn fun_mul(&self) -> &'static str;
	fn fun_div(&self) -> &'static str;
	fn fun_mod(&self) -> &'static str;

	fn fun_copy(&self) -> &'static str;
	fn fun_drop(&self) -> &'static str;

	fn act_print_integer(&self) -> &'static str;

}

pub struct MockBackend {}

impl Backend for MockBackend {
	fn complete(compiled: &str) -> String {
		compiled.to_string()
	}

	fn fun_add(&self) -> &'static str {
		""
	}

	fn fun_sub(&self) -> &'static str {
		""
	}

	fn fun_mul(&self) -> &'static str {
		""
	}

	fn fun_div(&self) -> &'static str {
		""
	}

	fn fun_mod(&self) -> &'static str {
		""
	}

	fn fun_copy(&self) -> &'static str {
		""
	}

	fn fun_drop(&self) -> &'static str {
		""
	}

	fn act_print_integer(&self) -> &'static str {
		""
	}

	fn create_function(&self, fname: &str, body: String) -> String {
		"".to_string()
	}

	fn user_function(&self, fname: &str) -> String {
		"".to_string()
	}

	fn push_integer(&self, i: &str) -> String {
		"".to_string()	
	}

}