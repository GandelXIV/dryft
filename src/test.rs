use crate::backends::c99::C99Backend;
use crate::backends::MockBackend;
use crate::frontend::compile;
use crate::backends::Backend;

fn make_strings(v: Vec<&str>) -> Vec<String> {
    v.into_iter().map(String::from).collect()
}

#[test]
#[cfg(feature = "typesystem")]
fn ts_primitive() {
    use std::panic;

    // this is supposed to crash
    let result = panic::catch_unwind(|| {
        let mut backend: Box<dyn Backend> = Box::new(MockBackend {});
        compile(&mut backend, "act main \"text\" 1 + :act")
    });
    println!("{:?}", result);
    assert!(result.is_err(), "Expected type error panic");
}

#[test]
#[cfg(feature = "typesystem")]
fn ts_variable_read() {
    use std::panic;

    // this is supposed to crash
    let result = panic::catch_unwind(|| {
        let mut backend: Box<dyn Backend> = Box::new(MockBackend {});
        compile(
            &mut backend,
            "act main \"hello\" var x 5 var y $x $y + :act",
        )
    });
    assert!(result.is_err(), "Expected type error panic");
}

#[test]
#[cfg(feature = "typesystem")]
fn ts_variable_write() {
    use std::panic;

    // this is supposed to crash
    let result = panic::catch_unwind(|| {
        let mut backend: Box<dyn Backend> = Box::new(MockBackend {});
        compile(&mut backend, "act main 1 var x \"str\" x! :act")
    });
    assert!(result.is_err(), "Expected type error panic");
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
