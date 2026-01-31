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

use crate::backends::c99::C99Backend;
use crate::backends::Backend;
use crate::backends::MockBackend;
use crate::frontend::compile;

fn make_strings(v: Vec<&str>) -> Vec<String> {
    v.into_iter().map(String::from).collect()
}

// handle specific result for CompilerState::throw_error()
#[cfg(feature = "typesystem")]
fn expect_dryft_err(code: &str, e: &str) {
    use std::panic;
    let result = panic::catch_unwind(|| {
        let mut backend: Box<dyn Backend> = Box::new(MockBackend {});
        compile(&mut backend, code);
    });
    assert_eq!(
        e.to_string(),
        *result.unwrap_err().downcast_ref::<String>().unwrap()
    );
}

#[test]
#[cfg(feature = "typesystem")]
fn ts_primitive() {
    expect_dryft_err(
        "act main \"text\" 1 + :act",
        "[DRYFT ERROR] <main>:1, word 4: Type mismatch : Expected Number, found Text",
    );
}

#[test]
#[cfg(feature = "typesystem")]
fn ts_variable_read() {
    expect_dryft_err(
        "act main \"hello\" var x 5 var y $x $y + :act",
        "[DRYFT ERROR] <main>:1, word 10: Type mismatch : Expected Number, found Text",
    );
}

#[test]
#[cfg(feature = "typesystem")]
fn ts_variable_write() {
    expect_dryft_err(
        "act main 1 var x \"str\" x! :act",
        "[DRYFT ERROR] <main>:1, word 6: Type mismatch : Expected Number, found Text",
    );
}

#[test]
#[cfg(feature = "typesystem")]
fn ts_function_inference() {
    expect_dryft_err(
        "fun: inc 1 + ; act main true inc ;",
        "[DRYFT ERROR] <main>:1, word 9: Type mismatch : Expected Number, found Binary",
    );
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
    let cs = compile(&mut backend, "fun: id ;");
    let mut backend: Box<dyn Backend> = Box::new(MockBackend {});
    let cs2 = compile(&mut backend, "fun: id :fun");
    assert_eq!(cs.out, cs2.out);
}

#[test]
fn function_compilation() {
    let mut backend: Box<dyn Backend> = Box::new(C99Backend {});
    let cs = compile(&mut backend, "fun: sum3 + + :fun");

    assert_eq!(
        cs.out.unwrap(),
        "void fun_sum3() { add(); add(); }\n".to_string()
    );
}

#[test]
fn strings() {
    let mut backend: Box<dyn Backend> = Box::new(C99Backend {});
    let cs = compile(&mut backend, "fun idk \" # fake comment # \" ; ");
}
