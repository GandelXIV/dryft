use regex::Regex;
use std::io::Write;    
use std::io;
use std::process::Command;
use std::collections::HashMap;

enum LiteralType {
    Number(isize),
    Text(String),
}

#[derive(Debug)]
enum MethodClass {
    Function,
    Action,
}

#[derive(Debug)]
struct Method {
    name: Option<String>,
    class: MethodClass,
    block: String,
}

enum Definition {
    Method(Method),
}


struct FrontendData {
    user_functions: HashMap<String, Method>,
}

trait Compiler {

    fn compile_program(&mut self, code: &str) -> String {
        let a = self.compile_block(code);
        return self.finalize(a) ;
    }
    
    fn finalize(&self, prod: String) -> String;
    fn push_literal(&mut self, lit: LiteralType) -> String;
    fn user_method(&mut self, name: &str) -> String;
    
    fn meth_add(&mut self) -> String;
    fn meth_mul(&mut self) -> String;
    fn meth_sub(&mut self) -> String;
    fn meth_div(&mut self) -> String;
    fn meth_mod(&mut self) -> String;
    fn meth_puti(&mut self) -> String;
    fn meth_puts(&mut self) -> String;
    fn fun_copy(&mut self) -> String;
    fn fun_drop(&mut self) -> String;
    
    fn defn_method(&mut self, m: &Method) -> String;
    
    fn compile_block(&mut self, code: &str) -> String {
        const SPACE: &str = " ";
        let mut buf = String::new();
        let re_int = Regex::new(r"^[0-9]+$").unwrap();
        let mut code = code.replace("\n", SPACE);
        let mut code = code.replace("\t", SPACE);
        let mut isdefinition = false;
        let mut isendofdefin = false;
        let mut isdefnaming  = false;
        let mut defstack: Vec<Definition> = vec![ ];
        let mut blockstack: Vec<String> = vec![String::new()];
        let mut fd = FrontendData { user_functions: HashMap::new() };
        for word in code.split(SPACE) {
            // println!("{}", &word);
            if isdefinition {
                match word {
                    "fun" => { 
                        defstack.push(Definition::Method(Method { name: None, class: MethodClass::Function, block: String::new() }));
                        blockstack.push(String::new());
                        isdefnaming = true;
                        
                    },
                    other => panic!("Unknown defintion {}", other), 
                }
                isdefinition = false;
                continue;
            }
            if isdefnaming {
                match defstack.last_mut().unwrap() {
                    Definition::Method(m) => { m.name = Some(word.to_string()); }
                }
                isdefnaming = false;
                continue;
            }
            let target = &match word {
                "" => {"".into()}
                ":" => { isdefinition = true; "".into() }
                ";" => { isendofdefin = true; "".into() }
                "+" => self.meth_add(),
                "*" => self.meth_mul(),
                "-" => self.meth_sub(),
                "/" => self.meth_div(),
                "mod" => self.meth_mod(),
                "puti" => self.meth_puti(),
                "puts" => self.meth_puts(),
                word if word.starts_with('"') && word.ends_with('"') => {
                    self.push_literal(LiteralType::Text(word.into()))
                }
                word if re_int.is_match(word) => self.push_literal(LiteralType::Number(word.parse::<isize>().unwrap())),
                custom => {
                    // println!("{}", custom);
                    // self.user_method(custom)
                    // println!("{} {:?}", word, &fd.user_functions);
                    println!("calling:{}", &custom);
                    self.user_method(word)
                }
            };
            if isendofdefin {
                let complete = defstack.pop().unwrap();
                match complete {
                    Definition::Method(mut meth) => {
                        let b = blockstack.pop().unwrap();                        
                        meth.block = b;
                        blockstack.last_mut().unwrap().push_str(&self.defn_method(&meth));
                        // TODO: add functions to handle list for inlining
                        match meth.class {
                            MethodClass::Function => {fd.user_functions.insert(meth.name.clone().expect("grrr"), meth);}
                            MethodClass::Action => {}
                        }

                    },
                };
                isendofdefin = false;
            } else {
                blockstack.last_mut().expect("idk").push_str(target);
            }
        }
        blockstack.last().unwrap().clone()
    }
}

struct C99Backend {}

impl Compiler for C99Backend {
    fn push_literal(&mut self, lit: LiteralType) -> String {
        match lit {
            LiteralType::Number(n) => format!("psh({});", n),
            LiteralType::Text(s) => format!("psh((size_t) {});", s)
        }
    }

    fn user_method(&mut self, name: &str) -> String {
        format!("{}();", name)
    }

    fn finalize (&self, prod: String) -> String {
        let mut base = include_str!("cbase.c").to_string();
        base.push_str(&format!("{}", &prod));
        return base;
    }

    fn meth_add( &mut self  ) -> String {
        "add();".to_string()
    }

    fn meth_mul( &mut self  ) -> String {
        "mul();".to_string()
    }

    fn meth_sub( &mut self  ) -> String {
        "sub();".to_string()
    }

    fn meth_div( &mut self  ) -> String {
        "div();".to_string()
    }

    fn meth_mod( &mut self  ) -> String {
        "mod();".to_string()
    }

    fn meth_puti(&mut self) -> String {
        "puti();".to_string()
    }

    fn meth_puts(&mut self) -> String {
        "putstr();".to_string()
    }

    fn fun_copy(&mut self) -> String {
        "copy();".to_string()
    }

    fn fun_drop(&mut self) -> String {
        "drop();".to_string()
    }

    fn defn_method(&mut self, m: &Method) -> String {
        format!("void {} (){{ {} }}\n", m.name.as_ref().unwrap(), m.block)
    }
}

mod tests {
    use crate::*;
    #[test]
    fn empty() {}

    
}

fn repl() {
    let mut backend = C99Backend {};
    loop {
        print!("Dryft repl> ");
        io::stdout().flush().unwrap();   // flush so it appears immediately
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        if input.trim().is_empty() {
            continue;
        }
        match input.trim() {
            "#open" => {
                let mut fname = String::new();  
                io::stdin()
                    .read_line(&mut fname)
                    .expect("Failed to read line");
                // println!("{}", &fname.trim());
                let src = &String::from_utf8(std::fs::read(fname.trim()).unwrap()).unwrap();
                build_and_run(&mut backend, src);
                continue;
            }
            _ => {}
        }
        build_and_run(&mut backend, &*input);
    }
}

fn build_and_run<B: Compiler>(backend: &mut B,c: &str) {
    std::fs::write(".temp.c", backend.compile_program(c)).unwrap();
    let output = Command::new("bash")
        .arg("-c")
        .arg("gcc .temp.c && ./a.out")
        .output()
        .expect("Failed to execute bash");
    println!("Out | {}", &String::from_utf8_lossy(&output.stdout));
    println!("Err | {}", &String::from_utf8_lossy(&output.stderr));
}

fn main() {
    repl();
    println!("Done!");
}
