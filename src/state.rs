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

use std::collections::HashMap;
use strum_macros::IntoStaticStr;

#[derive(Debug, PartialEq, IntoStaticStr)]
pub enum DefinitionTypes {
    Function,
    Action,
    Linkin,
    Then,
    Elect,
    Include,
    Loop,
    Variable,
    Module,
    Negative, // purely comparative, not actually constructed by code
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ValueTypes {
    Number,
    Text,
    Binary,
    Fake, // purely comparative, not actually constructed by code
}

#[derive(Debug)]
pub struct CompileState {
    pub out: Option<String>,     // access after compile() has been called
    pub log_tokens: Vec<String>, // purely for debugging usecases

    pub functions: HashMap<String, String>,
    pub actions: HashMap<String, String>,

    pub word: String,

    pub defnstack: Vec<DefinitionTypes>,
    pub metastack: Vec<Vec<String>>,
    pub bodystack: Vec<String>,
    pub varscopes: Vec<HashMap<String, ValueTypes>>,
    pub typestack: Vec<Vec<ValueTypes>>,
    pub voidstack: Vec<Vec<ValueTypes>>,

    pub iscomment: bool,
    pub isstring: bool,
    pub newstring: String,

    pub prepend: String,
    pub prepend_remaining: usize, // characters remaining from current prepended content

    pub linenumber: isize,
    pub tokenumber: isize,
    pub token_line: isize,  // line where current token started
    pub token_file: String, // file where current token started
    pub current_file: String,
    pub file_stack: Vec<(String, isize)>, // stack of (filename, line_number) for includes
}

impl CompileState {
    pub fn new() -> Self {
        Self {
            out: None,
            log_tokens: vec![],
            word: String::new(),
            functions: HashMap::new(),
            actions: HashMap::new(),

            defnstack: vec![],
            metastack: vec![],
            bodystack: vec![String::new()],
            varscopes: vec![HashMap::new()],
            typestack: vec![vec![]],
            voidstack: vec![vec![]],

            iscomment: false,
            isstring: false,
            newstring: String::new(),
            prepend: String::new(),
            prepend_remaining: 0,
            linenumber: 1,
            tokenumber: 0,
            token_line: 1,
            token_file: "<main>".to_string(),
            current_file: "<main>".to_string(),
            file_stack: vec![],
        }
    }

    // append codegen
    pub fn add2body(&mut self, s: &str) {
        self.bodystack.last_mut().unwrap().push_str(s)
    }

    pub fn push_type(&mut self, t: ValueTypes) {
        self.typestack.last_mut().unwrap().push(t)
    }

    pub fn pop_type(&mut self) -> ValueTypes {
        if cfg!(not(feature = "typesystem")) {
            return ValueTypes::Fake
        }
        
        self.typestack
            .last_mut()
            .expect("should implement pulling types from previous frame")
            .pop()
            .expect("type stack should not be empty when popping")            
    }

    pub fn expect_types(&mut self, expected: &[ValueTypes]) {
        if cfg!(not(feature = "typesystem")) {
            return
        }

        let stack = self
            .typestack
            .last_mut()
            .expect("type stack frame should exist");

        if stack.len() >= expected.len() {

            let mut actual_types = Vec::new();
            for _ in 0..expected.len() {
                actual_types.push(stack.pop().expect("type should exist on stack"));
            }
            actual_types.reverse();

            for (i, (&expected_type, &actual_type)) in
                expected.iter().zip(actual_types.iter()).enumerate()
            {
                if actual_type != expected_type && expected_type != ValueTypes::Fake {
                    self.throw_error(&format!(
                        "Type mismatch at position {}: expected {:?}, got {:?}",
                        i, expected_type, actual_type
                    ));
                }
            }
        }
    }

    // checks that the action is not called inside any function scope
    pub fn before_action(&self) {
        if self.defnstack.contains(&DefinitionTypes::Function) {
            self.throw_error(&format!("Can not call actions from inside a function"));
        }
    }

    // does the variable exist in scope? the actual location is handled by the backend
    pub fn variable_in_scope(&self, vname: &str) -> Option<ValueTypes> {
        for scope in self.varscopes.iter() {
            if scope.contains_key(vname) {
                return Some(scope.get(vname).unwrap().clone());
            }
        }
        None
    }

    pub fn grow_bodystack(&mut self) {
        self.bodystack.push("".into())
    }

    pub fn grow_varscopes(&mut self) {
        self.varscopes.push(HashMap::new())
    }

    pub fn grow_metastack(&mut self) {
        self.metastack.push(vec![])
    }

    pub fn grow_typestack(&mut self) {
        self.typestack.push(vec![])
    }

    pub fn grow_voidstack(&mut self) {
        self.typestack.push(vec![])
    }

    pub fn throw_error(&self, msg: &str) -> ! {
        let line = self.token_line;
        let token = self.tokenumber;
        let file = &self.token_file;
        panic!("[DRYFT ERROR] {file}:{line}, word {token}: {msg}")
    }

    pub fn throw_warning(&self, msg: &str) {
        let line = self.token_line;
        let token = self.tokenumber;
        let file = &self.token_file;
        println!("[DRYFT WARNING] {file}:{line}, word {token}: {msg}")
    }
}