use std::process::Command;
use std::io;
use std::io::Write;


pub mod backends;
pub mod frontend;

use backends::c99::C99Backend;
use backends::Backend;

fn repl() {
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
                build_and_run(src);
            }
            _ => {
                build_and_run(&*format!("fun: main {} ;", input));
            }
        }
    }
}

fn build_and_run(c: &str) {
    let mut backend = C99Backend {};
    std::fs::write(".temp.c", frontend::compile_full(backend, c)).unwrap();
    // move this into backend in the future
    let output = Command::new("bash")
        .arg("-c")
        .arg("gcc -w .temp.c && ./a.out")
        .output()
        .expect("Failed to execute bash");
    //println!("{}", &String::from_utf8_lossy(&output.stdout));
    println!("Out | {}", &String::from_utf8_lossy(&output.stdout));
    println!("Err | {}", &String::from_utf8_lossy(&output.stderr));
}


fn main() {
    repl();
}