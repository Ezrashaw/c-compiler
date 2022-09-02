// TODO: create some sort of TokenStream or TokenReader and maybe abstract the lexer into that (this could be done with `Iterator`)

// TODO: implement a tree for variables, instead of the bad hashmap system we currently use.

// TODO: create an IR

// TODO: function validition, we need new step with variable and function validation

use std::{env, fs, path::Path, process::Command};

use generator::Generator;
use lexer::Lexer;

use crate::parser::Parser;

mod ctypes;
mod generator;
mod lexer;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::read_to_string(&args[1]).unwrap();

    let mut lexer = Lexer::new(file.as_bytes());
    let tokens = lexer.read_all();

    #[cfg(debug_assertions)]
    {
        println!("=====TOKENS=====");
        for token in &tokens {
            println!("{:?}", token);
        }
    }

    let mut parser = Parser::new(&tokens);
    let ast = parser.read_program();

    #[cfg(debug_assertions)]
    {
        println!("=====AST=====");
        println!("{:?}", ast);
    }

    let generator = Generator::new(&ast);
    let asm = generator.gen_asm();

    #[cfg(debug_assertions)]
    {
        println!("=====ASM=====");
        println!("{}", asm);
    }

    fs::write("assembly.s", asm).unwrap();

    let path = Path::new(&args[1]).with_extension("");
    Command::new("gcc")
        .arg("assembly.s")
        .arg("-m32")
        .arg("-o")
        .arg(if cfg!(debug_assertions) {
            "output"
        } else {
            path.to_str().unwrap()
        })
        .spawn()
        .expect("gcc command failed to run")
        .wait()
        .unwrap();

    fs::remove_file("assembly.s").unwrap();
}
