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

pub mod c99;
pub mod nasm64;

pub trait Backend {
    fn complete(&self, compiled: &str) -> String;

    fn create_function(&self, fname: &str, body: String) -> String;
    fn push_integer(&self, i: &str) -> String;
    fn push_string(&self, s: &str) -> String;
    fn user_function(&self, f: &str) -> String; // CALL a user defined function
    fn linkin_function(&self, name: &str) -> String;

    fn fun_add(&self) -> &'static str;
    fn fun_sub(&self) -> &'static str;
    fn fun_mul(&self) -> &'static str;
    fn fun_div(&self) -> &'static str;
    fn fun_mod(&self) -> &'static str;

    fn fun_simple_equality(&self) -> &'static str;
    fn fun_simple_non_equality(&self) -> &'static str;

    fn fun_copy(&self) -> &'static str;
    fn fun_drop(&self) -> &'static str;
    fn fun_swap(&self) -> &'static str;

    fn fun_logical_not(&self) -> &'static str;
    fn fun_logical_and(&self) -> &'static str;
    fn fun_logical_or(&self) -> &'static str;

    fn fun_num_greater(&self) -> &'static str;

    fn create_then_condition(&self, body: String) -> String;
    fn create_else_condition(&self, body: String) -> String;

}

pub fn select(name: &str) -> Box<dyn Backend> {
    match name {
        "C99" => Box::new(c99::C99Backend {}),
        "NASM64" => Box::new(nasm64::Nasm64Backend {}),
        other => panic!("Invalid backend {other}"),
    }
}

pub struct MockBackend {}

impl Backend for MockBackend {
    
    fn create_else_condition(&self, body: String) -> String {
        "".to_string()
    }

    fn fun_num_greater(&self) -> &'static str {
        ""
    }

    fn fun_logical_not(&self) -> &'static str {
        ""
    }

    fn fun_logical_and(&self) -> &'static str {
        ""
    }

    fn fun_logical_or(&self) -> &'static str {
        ""
    }

    fn create_then_condition(&self, body: String) -> String {
        "".to_string()
    }

    fn fun_simple_equality(&self) -> &'static str {
        ""
    }

    fn fun_simple_non_equality(&self) -> &'static str {
        ""
    }

    fn fun_swap(&self) -> &'static str {
        ""
    }

    fn linkin_function(&self, name: &str) -> String {
        "".to_string()
    }

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

    fn create_function(&self, fname: &str, body: String) -> String {
        "".to_string()
    }

    fn user_function(&self, fname: &str) -> String {
        "".to_string()
    }

    fn push_integer(&self, i: &str) -> String {
        "".to_string()
    }

    fn push_string(&self, s: &str) -> String {
        "".to_string()
    }
}
