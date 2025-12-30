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

pub struct C99Backend {}

impl Backend for C99Backend {
    fn write_variable(&self, name: &str) -> String {
        format!("var_{name} = dryft_pop(); ")
    }

    fn read_variable(&self, name: &str) -> String {
        format!("dryft_push(var_{name}); ")
    }

    fn create_variable(&self, name: &str) -> String {
        format!("size_t var_{name} = dryft_pop(); ")
    }

    fn loop_break(&self) -> String {
        "break;".to_string()
    }

    fn fun_num_greater(&self) -> &'static str {
        "num_greater(); "
    }

    fn fun_logical_not(&self) -> &'static str {
        "logical_not(); "
    }

    fn fun_logical_and(&self) -> &'static str {
        "logical_and(); "
    }

    fn fun_logical_or(&self) -> &'static str {
        "logical_or(); "
    }

    fn create_then_condition(&self, body: String) -> String {
        format!("if (dryft_pop()) {{\n\tlast_condition = true;\n{body}\n}} else {{\t\nlast_condition = false;\n}}")
    }

    fn create_else_condition(&self, body: String) -> String {
        format!("if (!last_condition) {{\n\t{body}\n}}")
    }

    fn create_loop_block(&self, body: String) -> String {
        format!("for (;;) {{\n\t{body}\n}}")
    }

    fn complete(&self, compiled: &str) -> String {
        let mut cbase = include_str!("c99base.c").to_string();
        cbase.push_str(compiled);
        cbase
    }

    fn fun_simple_equality(&self) -> &'static str {
        "simple_equality(); "
    }

    fn fun_simple_non_equality(&self) -> &'static str {
        "simple_non_equality(); "
    }

    fn fun_swap(&self) -> &'static str {
        "swap(); "
    }

    fn linkin_function(&self, name: &str) -> String {
        // because we prepend all user functions with fun_ , we have to get to the linked function indirectly
        format!("extern void {name}();\nvoid fun_{name}(){{ {name}(); }}\n")
    }

    // TODO: hash function names to avoid clashes with internals and allow symbol only names
    fn create_function(&self, fname: &str, body: String) -> String {
        format!("void fun_{}() {{ {}}}\n", fname, body)
    }

    fn user_function(&self, fname: &str) -> String {
        format!("fun_{fname}(); ")
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

    fn push_integer(&self, i: &str) -> String {
        format!("dryft_push({i}); ")
    }

    fn push_string(&self, s: &str) -> String {
        format!("dryft_push(\"{s}\"); ")
    }
}
