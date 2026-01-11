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
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
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
    Negative, // purely comparative for compiler purposes
}

pub struct CompileState {
    pub out: Option<String>,     // access after compile() has been called
    pub log_tokens: Vec<String>, // purely for debugging usecases

    pub functions: HashMap<String, String>,
    pub actions: HashMap<String, String>,

    pub word: String,

    pub defnstack: Vec<DefinitionTypes>,
    pub metastack: Vec<Vec<String>>,
    pub bodystack: Vec<String>,
    pub varscopes: Vec<HashSet<String>>,

    pub iscomment: bool,
    pub isstring: bool,
    pub newstring: String,

    pub prepend: String,

    pub linenumber: isize,
    pub tokenumber: isize,
}

impl CompileState {
    fn new() -> Self {
        Self {
            out: None,
            log_tokens: vec![],
            word: String::new(),
            functions: HashMap::new(),
            actions: HashMap::new(),
            defnstack: vec![],
            metastack: vec![],
            bodystack: vec![String::new()],
            varscopes: vec![HashSet::new()],
            iscomment: false,
            isstring: false,
            newstring: String::new(),
            prepend: String::new(),
            linenumber: 0,
            tokenumber: 0,
        }
    }

    // append codegen
    fn add2body(&mut self, s: &str) {
        self.bodystack.last_mut().unwrap().push_str(s)
    }

    // checks that the action is not called inside any function scope
    fn before_action(&self) {
        if self.defnstack.contains(&DefinitionTypes::Function) {
            self.throw_error(&format!("Can not call actions from inside a function"));
        }
    }

    // does the variable exist in scope? the actual location is handled by the backend
    fn variable_in_scope(&self, vname: &str) -> bool {
        for scope in self.varscopes.iter() {
            if scope.contains(vname) {
                return true;
            }
        }
        false
    }

    fn grow_bodystack(&mut self) {
        self.bodystack.push("".into())
    }

    fn grow_varscopes(&mut self) {
        self.varscopes.push(HashSet::new())
    }

    fn grow_metastack(&mut self) {
        self.metastack.push(vec![])
    }

    fn throw_error(&self, msg: &str) -> ! {
        let line = self.linenumber;
        let token = self.tokenumber; // token printing is broken when includes are used, figure out why TODO
        panic!("[DRYFT ERROR] line {line}, word {token}: {msg}")
    }
}

pub fn compile(backend: &mut Box<dyn Backend>, code: &str) -> CompileState {
    let mut cs = CompileState::new();

    macro_rules! new_token {
        () => {{
            if !cs.word.is_empty() {
                cs.tokenumber += 1;
                cs.log_tokens.push(cs.word.clone());
                handle_token(backend, &mut cs);
                cs.word = String::new();
            }
        }};
    }

    let mut code = code.to_string();

    while !code.is_empty() {
        if !cs.prepend.is_empty() {
            code.insert_str(0, &cs.prepend);
            cs.prepend = String::new();
        }

        let letter = code.remove(0);

        if letter == '\n' {
            cs.linenumber += 1;
            cs.tokenumber = 0;
        }

        match letter {
            c if cs.iscomment => {
                if c == '#' {
                    cs.iscomment = false;
                }
            }
            c if cs.isstring => {
                if c == '"' {
                    cs.isstring = false;
                    cs.add2body(&backend.push_string(&cs.newstring));
                    cs.newstring = String::new();
                } else {
                    cs.newstring.push(c);
                }
            }
            ' ' | '\n' | '\t' => new_token!(),
            '#' => cs.iscomment = true,
            '"' => cs.isstring = true,
            other => cs.word.push(other),
        }
    }
    new_token!(); // last word may not be whitespace separated
    cs.out = Some(cs.bodystack.remove(0));

    cs
}

pub fn compile_full(mut backend: Box<dyn Backend>, code: &str) -> String {
    let built = compile(&mut backend, code)
        .out
        .expect("No code compiled :(");
    backend.complete(&built)
}

fn handle_token(backend: &mut Box<dyn Backend>, cs: &mut CompileState) {
    let regexint = Regex::new(r"^-?\d+$").unwrap();

    // this should actually only be used for defintions that need their own body and meta stack :C, allocating a new body is unnecessary otherwise
    macro_rules! new_definition {
        ($variant:ident) => {{
            cs.defnstack.push(DefinitionTypes::$variant);
            cs.metastack.push(vec![]);
            cs.bodystack.push(String::new());
        }};
    }

    macro_rules! add_function {
        () => {
            let meta = cs.metastack.pop().unwrap();
            let body = cs.bodystack.pop().unwrap();

            let fname = meta
                .get(0)
                .unwrap_or_else(|| cs.throw_error("No function name provided"));

            if fname == "main" {
                cs.throw_error("main must be defined as an action")
            }

            cs.varscopes.pop();
            cs.functions.insert(fname.clone(), body.clone());

            let f = backend.create_function(fname.as_ref(), body);
            cs.add2body(&f);
        };
    }

    macro_rules! add_action {
        () => {
            let meta = cs.metastack.pop().unwrap();
            let body = cs.bodystack.pop().unwrap();

            let aname = meta
                .get(0)
                .unwrap_or_else(|| cs.throw_error("No function name provided"));

            cs.varscopes.pop();
            cs.actions.insert(aname.clone(), body.clone());

            let f = backend.create_function(aname.as_ref(), body);
            cs.add2body(&f);
        };
    }

    macro_rules! add_then_block {
        () => {
            let body = cs.bodystack.pop().unwrap();
            cs.varscopes.pop();

            let inelect = cs.defnstack.last().unwrap() == &DefinitionTypes::Elect;

            cs.add2body(&backend.create_conditional_statement(body, inelect));
        };
    }
    macro_rules! add_elect_block {
        () => {
            let body = cs.bodystack.pop().unwrap();
            cs.add2body(&backend.create_elect_block(body));
        };
    }

    macro_rules! add_loop_block {
        () => {
            let body = cs.bodystack.pop().unwrap();
            cs.varscopes.pop();

            cs.add2body(&backend.create_loop_block(body));
        };
    }

    macro_rules! add_builtin {
        ($prop:ident) => {{
            cs.add2body(backend.$prop())
        }};
    }

    macro_rules! check_terminator {
        ($expected:ident) => {
            if cs
                .defnstack
                .pop()
                .unwrap_or_else(|| cs.throw_error("no block to end"))
                != DefinitionTypes::$expected
            {
                cs.throw_error(concat!(
                    "Misplaced ",
                    stringify!($expected),
                    " block ending"
                ));
            }
        };
    }

    match cs.word.as_ref() {
        // needs higher priority than fun & act keywords
        x if *cs.defnstack.last().unwrap_or(&DefinitionTypes::Negative)
            == DefinitionTypes::Linkin
            && cs.metastack.last().unwrap().len() < 2 =>
        {
            cs.metastack.last_mut().unwrap().push(x.into());
            // if we have all the arguments we needed
            if cs.metastack.last_mut().unwrap().len() == 2 {
                let mut meta = cs.metastack.pop().unwrap();
                cs.defnstack.pop(); // end our defintion

                let class = meta.remove(0);
                let mname = meta.remove(0);

                match class.as_ref() {
                    "fun" => cs.functions.insert(mname.clone(), "LINKED IN".to_string()),
                    "act" => cs.actions.insert(mname.clone(), "LINKED IN".to_string()),
                    other => cs.throw_error(&format!("Invalid link-in class {other}")),
                };

                cs.add2body(&backend.linkin_function(&mname));
            }
        }

        f if *cs.defnstack.last().unwrap_or(&DefinitionTypes::Negative)
            == DefinitionTypes::Include =>
        {
            cs.defnstack.pop();
            let mut pat = String::from(f);
            pat.push_str(".dry");
            let f = String::from_utf8(fs::read(&pat).expect("Could not locate include")).unwrap();
            cs.prepend.push_str(&f);
            cs.linenumber -= f.matches('\n').count() as isize;
        }

        v if *cs.defnstack.last().unwrap_or(&DefinitionTypes::Negative)
            == DefinitionTypes::Variable =>
        {
            cs.defnstack.pop();
            let vname = v;

            if cs.functions.contains_key(vname)
                || cs.actions.contains_key(vname)
                || cs.variable_in_scope(vname)
            {
                cs.throw_error(&format!(
                    "cant define variable, symbol {vname} is already taken"
                ))
            }

            cs.varscopes.last_mut().unwrap().insert(vname.to_string());
            cs.add2body(&backend.create_variable(vname));
        }

        "fun:" | "fun" => {
            new_definition!(Function);
            cs.varscopes.push(HashSet::new());
        }

        ":fun" => {
            check_terminator!(Function);
            add_function!();
        }

        "act:" | "act" => {
            new_definition!(Action);
            cs.varscopes.push(HashSet::new());
        }

        ":act" => {
            check_terminator!(Action);
            add_action!();
        }

        // this keyword is funamentally unsafe, consider adding changing to unsafe_linkin or something like that
        "linkin" => {
            cs.defnstack.push(DefinitionTypes::Linkin);
            cs.grow_metastack();
        }

        "include" | "include:" => {
            cs.defnstack.push(DefinitionTypes::Include);
        }

        "then" | "then:" => {
            cs.defnstack.push(DefinitionTypes::Then);
            cs.grow_bodystack();
            cs.grow_varscopes();
        }

        ":then" => {
            check_terminator!(Then);
            add_then_block!();
        }

        "elect" | "elect:" => {
            cs.defnstack.push(DefinitionTypes::Elect);
            cs.grow_bodystack();
        }

        ":elect" => {
            add_elect_block!();
        }

        "loop" | "loop:" => {
            cs.defnstack.push(DefinitionTypes::Loop);
            cs.grow_bodystack();
        }

        ":loop" => {
            check_terminator!(Loop);
            add_loop_block!();
            cs.varscopes.push(HashSet::new());
        }

        "break" => {
            cs.add2body(&backend.loop_break());
        }

        "return" => {
            cs.add2body(&backend.method_return());
        }

        "var" => {
            cs.defnstack.push(DefinitionTypes::Variable);
        }

        ";" | "end" => {
            match cs
                .defnstack
                .pop()
                .unwrap_or_else(|| cs.throw_error(" - Misplaced ;"))
            {
                // keep {} notation instead of , for the macros to work
                DefinitionTypes::Function => {
                    add_function!();
                }
                DefinitionTypes::Action => {
                    add_action!();
                }
                DefinitionTypes::Then => {
                    add_then_block!();
                }
                DefinitionTypes::Loop => {
                    add_loop_block!();
                }
                DefinitionTypes::Elect => {
                    add_elect_block!();
                }
                _ => todo!(),
            }
        }

        _mac if (false) => {} // future for macro expansion

        // TODO: optimize this into an argcount stack, which decrements top on each metastack push
        // TODO ALT basically all of these just grab x args, but maybe in the future they will also perform immediate work with them, so who knows actually?
        // TODO ALT just break this into a bunch of macro for ease of use & reading
        fname
            if *cs.defnstack.last().unwrap_or(&DefinitionTypes::Negative)
                == DefinitionTypes::Function
                && cs.metastack.last().unwrap().is_empty() =>
        {
            cs.metastack.last_mut().unwrap().push(fname.into())
        }

        aname
            if *cs.defnstack.last().unwrap_or(&DefinitionTypes::Negative)
                == DefinitionTypes::Action
                && cs.metastack.last().unwrap().is_empty() =>
        {
            cs.metastack.last_mut().unwrap().push(aname.into())
        }

        // body code must start here, as to not be confused for meta code
        fun if cs.functions.contains_key(fun) => {
            cs.add2body(&backend.user_function(fun));
        }

        act if cs.actions.contains_key(act) => {
            cs.before_action();
            cs.add2body(&backend.user_function(act));
        }

        var if var.starts_with('$') => {
            let vname = var.strip_prefix('$').unwrap();
            if cs.variable_in_scope(vname) {
                cs.add2body(&backend.read_variable(vname));
            } else {
                cs.throw_error(&format!("Variable '{vname}' not in scope"))
            }
        }

        num if regexint.is_match(num) => cs.add2body(&backend.push_integer(num)),

        setvar if setvar.ends_with('!') => {
            let vname = setvar.strip_suffix('!').unwrap();
            if !cs.variable_in_scope(vname) {
                cs.throw_error(&format!("Invalid write to variable {vname}, not found"))
            }
            cs.add2body(&backend.write_variable(vname));
        }

        "+" => add_builtin!(fun_add),
        "-" => add_builtin!(fun_sub),
        "*" => add_builtin!(fun_mul),
        "/" => add_builtin!(fun_div),
        "mod" => add_builtin!(fun_mod),
        "^" | "copy" => add_builtin!(fun_copy),
        "v" | "drop" => add_builtin!(fun_drop),
        "swap" => add_builtin!(fun_swap),
        "equals?" | "=?" => add_builtin!(fun_simple_equality),
        "nequals?" => add_builtin!(fun_simple_non_equality),
        "not" => add_builtin!(fun_logical_not),
        "either?" => add_builtin!(fun_logical_or),
        "both?" => add_builtin!(fun_logical_and),
        "greater?" | ">?" => add_builtin!(fun_num_greater),
        ">=?" => add_builtin!(fun_num_greater_or_equal),
        "<?" => add_builtin!(fun_num_less_than),
        "=<?" => add_builtin!(fun_num_less_than_or_equal),

        word => cs.throw_error(&format!("Unknown token '{}'", word)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backends::c99::C99Backend;
    use crate::backends::MockBackend;

    fn make_strings(v: Vec<&str>) -> Vec<String> {
        v.into_iter().map(String::from).collect()
    }

    #[test]
    fn simple_parse() {
        let mut backend: Box<dyn Backend> = Box::new(MockBackend {});
        let mut cs = compile(&mut backend, "fun: inc\n\t1 + :fun");
        assert_eq!(
            cs.log_tokens,
            make_strings(vec!["fun:", "inc", "1", "+", ":fun"])
        );
        //assert_eq!(cs.defstack, vec![(DefinitionTypes::Function, make_strings(vec!["inc", "1", "+", ";"]))]);
    }

    #[test]
    fn semicolon_ending() {
        let mut backend: Box<dyn Backend> = Box::new(MockBackend {});
        let mut cs = compile(&mut backend, "fun: id ;");
        let mut backend: Box<dyn Backend> = Box::new(MockBackend {});
        let mut cs2 = compile(&mut backend, "fun: id :fun");
        assert_eq!(cs.out, cs2.out);
    }

    #[test]
    fn function_compilation() {
        let mut backend: Box<dyn Backend> = Box::new(C99Backend {});
        let mut cs = compile(&mut backend, "fun: sum3 + + :fun");

        assert_eq!(
            cs.out.unwrap(),
            "void fun_sum3() { add(); add(); }\n".to_string()
        );
    }

    #[test]
    fn strings() {
        let mut backend: Box<dyn Backend> = Box::new(C99Backend {});
        let mut cs = compile(&mut backend, "fun idk \" # fake comment # \" ; ");
    }
}
