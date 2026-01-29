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
use crate::state::CompileState;
use crate::state::DefinitionTypes;
use crate::state::ValueTypes;
use regex::Regex;
use std::fs;

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

    // drain the whole text buffer character ny character
    // compilation is single-pass
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

        // Track line and file where current token starts
        if cs.word.is_empty() && !matches!(letter, ' ' | '\n' | '\t' | '#' | '"') {
            cs.token_line = cs.linenumber;
            cs.token_file = cs.current_file.clone();
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
                    cs.push_type(ValueTypes::Text);
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

        // Track when we've finished processing an included file
        if cs.prepend_remaining > 0 {
            cs.prepend_remaining -= 1;
            if cs.prepend_remaining == 0 && !cs.file_stack.is_empty() {
                let (prev_file, prev_line) = cs.file_stack.pop().unwrap();
                cs.current_file = prev_file;
                cs.linenumber = prev_line;
                cs.tokenumber = 0;
                // Don't reset token_line here - it will be updated naturally when next token starts
            }
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
            let _ts = cs.typestack.pop().unwrap();

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
            let _ts = cs.typestack.pop().unwrap();

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
            //cs.varscopes.pop();

            cs.add2body(&backend.create_loop_block(body));
        };
    }

    macro_rules! add_builtin {
        ($prop:ident) => {{
            cs.add2body(backend.$prop())
        }};
    }

    macro_rules! add_module {
        () => {
            let _body = cs.bodystack.pop().unwrap();
        };
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

    match cs.word.clone().as_ref() {
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
            let included_content =
                String::from_utf8(fs::read(&pat).expect("Could not locate include")).unwrap();

            // Save current file context
            cs.file_stack
                .push((cs.current_file.clone(), cs.linenumber - 1));
            cs.current_file = pat.clone();
            cs.linenumber = 1; // Start at line 1 for the included file
            cs.tokenumber = 0;

            // Track length of included content so we know when it's done
            cs.prepend_remaining = included_content.len();
            cs.prepend.push_str(&included_content);
        }

        v if *cs.defnstack.last().unwrap_or(&DefinitionTypes::Negative)
            == DefinitionTypes::Variable =>
        {
            cs.defnstack.pop();
            let vname = v;

            if cs.functions.contains_key(vname)
                || cs.actions.contains_key(vname)
                || cs.variable_in_scope(vname).is_some()
            {
                cs.throw_error(&format!(
                    "cant define variable, symbol {vname} is already taken"
                ))
            }

            let vtype = cs.pop_type();

            cs.varscopes
                .last_mut()
                .unwrap()
                .insert(vname.to_string(), vtype);
            cs.add2body(&backend.create_variable(vname));
        }

        "fun:" | "fun" => {
            new_definition!(Function);
            cs.grow_varscopes();
            cs.grow_typestack();
            cs.grow_voidstack();
        }

        ":fun" => {
            check_terminator!(Function);
            add_function!();
        }

        "act:" | "act" => {
            new_definition!(Action);
            cs.grow_varscopes();
            cs.grow_typestack();
            cs.grow_voidstack();
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

        "elect" | "elect:" | "when" | "when:" => {
            cs.defnstack.push(DefinitionTypes::Elect);
            cs.grow_bodystack();
        }

        ":elect" | ":when" => {
            add_elect_block!();
        }

        "loop" | "loop:" | "cycle" | "cycle:" => {
            cs.defnstack.push(DefinitionTypes::Loop);
            cs.grow_bodystack();
        }

        ":loop" | ":cycle" => {
            check_terminator!(Loop);
            add_loop_block!();
            cs.grow_varscopes();
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

        "module" => {
            cs.defnstack.push(DefinitionTypes::Module);
            cs.grow_bodystack();
            cs.grow_metastack();
        }

        ":module" => {
            check_terminator!(Module);
            add_module!();
        }

        "struct" => {}

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
                DefinitionTypes::Module => {
                    add_module!();
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
            if let Some(t) = cs.variable_in_scope(vname) {
                cs.push_type(t);
                cs.add2body(&backend.read_variable(vname));
            } else {
                cs.throw_error(&format!("Variable '{vname}' not in scope"))
            }
        }

        num if regexint.is_match(num) => {
            cs.add2body(&backend.push_integer(num));
            cs.push_type(ValueTypes::Number);
        }

        setvar if setvar.ends_with('!') => {
            let vname = setvar.strip_suffix('!').unwrap();
            if let Some(t) = cs.variable_in_scope(vname) {
                cs.add2body(&backend.write_variable(vname));
                cs.expect_types(&[t])
            } else {
                cs.throw_error(&format!("Invalid write to variable {vname}, not found"))
            }
        }

        "+" => {
            cs.expect_types(&[ValueTypes::Number, ValueTypes::Number]);
            add_builtin!(fun_add);
            cs.push_type(ValueTypes::Number);
        }
        "-" => {
            cs.expect_types(&[ValueTypes::Number, ValueTypes::Number]);
            add_builtin!(fun_sub);
            cs.push_type(ValueTypes::Number);
        }
        "*" => {
            cs.expect_types(&[ValueTypes::Number, ValueTypes::Number]);
            add_builtin!(fun_mul);
            cs.push_type(ValueTypes::Number);
        }
        "/" => {
            cs.expect_types(&[ValueTypes::Number, ValueTypes::Number]);
            add_builtin!(fun_div);
            cs.push_type(ValueTypes::Number);
        }
        "mod" => {
            cs.expect_types(&[ValueTypes::Number, ValueTypes::Number]);
            add_builtin!(fun_mod);
            cs.push_type(ValueTypes::Number);
        }
        "^" | "copy" => {
            let t = cs.pop_type();
            add_builtin!(fun_copy);
            cs.push_type(t);
            cs.push_type(t);
        }
        "v" | "drop" => {
            let _t = cs.pop_type();
            add_builtin!(fun_drop);
        }
        "swap" => {
            let t1 = cs.pop_type();
            let t2 = cs.pop_type();
            add_builtin!(fun_swap);
            cs.push_type(t1);
            cs.push_type(t2);
        }
        "equals?" | "=?" => {
            let _t2 = cs.pop_type();
            let _t1 = cs.pop_type();
            add_builtin!(fun_simple_equality);
            cs.push_type(ValueTypes::Binary);
        }
        "nequals?" => {
            let _t2 = cs.pop_type();
            let _t1 = cs.pop_type();
            add_builtin!(fun_simple_non_equality);
            cs.push_type(ValueTypes::Binary);
        }
        "not" => {
            cs.expect_types(&[ValueTypes::Binary]);
            add_builtin!(fun_logical_not);
            cs.push_type(ValueTypes::Binary);
        }
        "either?" => {
            cs.expect_types(&[ValueTypes::Binary, ValueTypes::Binary]);
            add_builtin!(fun_logical_or);
            cs.push_type(ValueTypes::Binary);
        }
        "both?" => {
            cs.expect_types(&[ValueTypes::Binary, ValueTypes::Binary]);
            add_builtin!(fun_logical_and);
            cs.push_type(ValueTypes::Binary);
        }
        "greater?" | ">?" => {
            cs.expect_types(&[ValueTypes::Number, ValueTypes::Number]);
            add_builtin!(fun_num_greater);
            cs.push_type(ValueTypes::Binary);
        }
        ">=?" => {
            cs.expect_types(&[ValueTypes::Number, ValueTypes::Number]);
            add_builtin!(fun_num_greater_or_equal);
            cs.push_type(ValueTypes::Binary);
        }
        "<?" => {
            cs.expect_types(&[ValueTypes::Number, ValueTypes::Number]);
            add_builtin!(fun_num_less_than);
            cs.push_type(ValueTypes::Binary);
        }
        "=<?" => {
            cs.expect_types(&[ValueTypes::Number, ValueTypes::Number]);
            add_builtin!(fun_num_less_than_or_equal);
            cs.push_type(ValueTypes::Binary);
        }

        word => cs.throw_error(&format!("Unknown token '{}'", word)),
    }
}
