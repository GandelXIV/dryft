pub mod c99;

pub trait Backend {
	fn complete(compiled: &str) -> String;
}

pub struct MockBackend {}

impl Backend for MockBackend {
	fn complete(compiled: &str) -> String {
		"".to_string()
	}
}