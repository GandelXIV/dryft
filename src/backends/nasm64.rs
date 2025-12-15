/*
* Copyright (C) 2025 Filip Chovanec
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU General Public License as published by
* the Free Software Foundation, either version 3 of the License, or
* (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU General Public License for more details.
*
* You should have received a copy of the GNU General Public License
* along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use crate::backends::Backend;

pub struct Nasm64Backend {}

impl Backend for Nasm64Backend {
	fn complete(&self, compiled: &str) -> String {
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

	fn act_print_string(&self) -> &'static str { 
		"" 
	}

	fn push_string(&self, s: &str) -> String {
		"".to_string()
	}
}