pub mod c99;

pub trait Backend {
	fn complete( compiled: &str) -> String;

	fn fun_add(&self) -> &'static str;

	fn create_function(&self, fname: &str, body: String) -> String;
}

pub struct MockBackend {}

impl Backend for MockBackend {
	fn complete(compiled: &str) -> String {
		compiled.to_string()
	}

	fn fun_add(&self) -> &'static str {
		""
	}

	fn create_function(&self, fname: &str, body: String) -> String {
		"".to_string()
	}

}