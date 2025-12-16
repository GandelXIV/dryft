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
		let mut base = include_str!("nasm64base.asm").to_string();
		base.push_str(compiled);
		base	
	}

	fn fun_add(&self) -> &'static str {
		"\tcall builtin_add\n"
	}

	fn fun_sub(&self) -> &'static str {
		"\tcall builtin_sub\n"	
	}

	fn fun_mul(&self) -> &'static str {
		"\tcall builtin_mul\n"
	}

	fn fun_div(&self) -> &'static str {
		"\tcall builtin_div\n"
	}

	fn fun_mod(&self) -> &'static str {
		"\tcall builtin_mod\n"
	}

	fn fun_copy(&self) -> &'static str {
		"\tcall data_copy\n"
	}

	fn fun_drop(&self) -> &'static str {
		"\tcall data_pop\n"
	}

	fn act_print_integer(&self) -> &'static str {
		// TODO: implement this multi-digit numbers
		"\tcall builtin_print_digit\n"
	}

	fn create_function(&self, fname: &str, body: String) -> String {
		format!("fun_{fname}:\n{body}\tret\n\n")
	}

	fn user_function(&self, fname: &str) -> String {
		format!("\tcall fun_{fname}\n")
	}

	fn push_integer(&self, i: &str) -> String {
		format!("\tmpush {i}\n")	
	}

	fn act_print_string(&self) -> &'static str { 
		todo!()
	}

	fn push_string(&self, s: &str) -> String {
		todo!()
	}
}