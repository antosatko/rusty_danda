use std::{env, fs::File, io::Read, hint::black_box, time::{Duration, SystemTime}};
use ast_parser::ast_parser::generate_ast;
use tree_walker::tree_walker::generate_tree;
//use runtime::*;
// use runtime_types::*;
// use reader::reader::*;
// use crate::test::test::test_init;

mod runtime;
mod reader;
mod ast_parser;
mod lexer;
mod test;
mod token_refactor;
mod writer;
mod ast_analyzer;
mod tree_walker;

fn main() {
    let mut args = env::args();
    let path = match args.nth(0) {
        Some(path) => path,
        None => panic!("Path not specified."),
    };
    let cmd = match args.nth(0) {
        Some(cmd) => cmd,
        None => String::from(""),
    };

    match cmd.as_str() {
        "build" => {
            let file = match args.nth(0) {
                Some(file) => file,
                None => panic!("File not specified."),
            };
            println!("Compilation for '{file}' starts.");
            let mut string = String::new();
            let mut file =
                File::open(file).expect(&format!("File not found. ({})", path).to_owned());
            file.read_to_string(&mut string).expect("neco se pokazilo");
            use lexer::tokenizer::*;
            let ast = if let Some(ast) = generate_ast("ast/test.ast") {
                ast
            }else {
                panic!();
            };
            println!("AST loaded.");
            let time = SystemTime::now();
            let tokens = tokenize(string, true);
            println!("Lexing complete.");
            println!("time: {}", SystemTime::now().duration_since(time).unwrap().as_millis());
            let parsed_tree = generate_tree(&tokens.0, &ast, true   );
            println!("Parsed.");
            /*use std::io::*;
            let mut input = String::new();
            stdin().read_line(&mut input).expect("error: unable to read user input");
            println!("{}",input);*/
            black_box(parsed_tree);
        }
        "tokenize" => {
            let file = match args.nth(0) {
                Some(file) => file,
                None => panic!("File not specified."),
            };
            println!("Compilation for '{file}' starts.");
            let mut string = String::new();
            let mut file =
                File::open(file).expect(&format!("File not found. ({})", path).to_owned());
            file.read_to_string(&mut string).expect("neco se pokazilo");
            use lexer::tokenizer::*;
            let tokens = tokenize(string, true);
            println!("{:?}", tokens.0);
        }
        "astTest" => {
            let mut file_name = String::from("ast/");
            match args.nth(0) {
                Some(file) => file_name.push_str(&file),
                None => {
                    println!("file not specified");
                    return;
                }
            };
            if let Some(ast) = generate_ast(&file_name) {
                   for node in ast {
                    println!("{:?}\n", node)
                   }
            }else{
                println!("failed to parse AST properly")
            }
        }
        _ => {
            /*println!("{:?} == {:?} = {:?}", Some(56), None::<i32>, None == Some(56));
            println!("{:?} == {:?} = {:?}", Some(56), Some(92), Some(56) == Some(92));
            println!("{:?} == {:?} = {:?}", Some(56), Some(56), Some(56) == Some(56));
            println!("Unknown command: {}", cmd);
            println!("Try help.");*/
            let mut vector = Vec::with_capacity(5000000);
            let time = SystemTime::now();
            for _ in 0..5000000 {
                vector.push(String::from("5u128"));
            }
            println!("time: {}", SystemTime::now().duration_since(time).unwrap().as_millis());
            black_box(vector);
        }
    }
}
