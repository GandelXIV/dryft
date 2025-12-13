use crate::backends::Backend;

pub struct C99Backend {}

impl Backend for C99Backend {
	fn complete(compiled: &str) -> String {
		let mut cbase = include_str!("cbase.c").to_string();
		cbase.push_str(compiled);
		cbase
	}
}