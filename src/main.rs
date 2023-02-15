use ast_parser::ast_parser::generate_ast;
use std::{env, fs::File, hint::black_box, io::Read, os::windows::thread, time::SystemTime};
use tree_walker::tree_walker::generate_tree;
//use runtime::*;
// use runtime_types::*;
// use reader::reader::*;
// use crate::test::test::test_init;

mod ast_analyzer;
mod ast_parser;
mod lexer;
mod reader;
mod runtime;
mod test;
mod token_refactor;
mod tree_walker;
mod writer;

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
            let mut string = string.into_bytes();
            use lexer::tokenizer::*;
            let ast = if let Some(ast) = generate_ast("ast/default.ast") {
                ast
            } else {
                panic!();
            };
            println!("AST loaded.");
            let time = SystemTime::now();
            /*let mut last = 0;
            let mut slices: Vec<Vec<u8>> = Vec::new();
            const SEGMENTS: usize = 100000000;
            while string.len() > last + SEGMENTS {
                let this =
                last + SEGMENTS + lexer::tokenizer::get_token(&string[last + SEGMENTS..]).1;
                slices.push(string[last..this].to_vec());
                last = this;
            }
            slices.push(string[last..].to_vec());
            println!("slices done, duration since start: {}", SystemTime::now().duration_since(time).unwrap().as_millis());
            let mut parts = Vec::new();
            for _ in 0..slices.len() {
                parts.push(None);
            }
            println!("parts done, duration since start: {}", SystemTime::now().duration_since(time).unwrap().as_millis());
            use std::thread;
            let mut handles = Vec::with_capacity(parts.len());
            while let Some(slice) = slices.pop() {
                handles.push(thread::spawn(move || {
                    return tokenize(&slice, false);
                }));
            }
            println!("handles setuppped ({}), duration since start: {}", handles.len(), SystemTime::now().duration_since(time).unwrap().as_millis());
            while let Some(handle) = handles.pop() {
                let part = handle.join().expect("sedm");
                parts[handles.len()] = Some(part);
            }
            println!("handles joined, duration since start: {}", SystemTime::now().duration_since(time).unwrap().as_millis());
            let mut tokens = (Vec::new(), Vec::new(), Vec::new());
            while let Some(Some(mut part)) = parts.pop() {
                tokens.0.append(&mut part.0);
                tokens.1.append(&mut part.1);
                tokens.2.append(&mut part.2);
            }*/
            let mut tokens = tokenize(&string, false);
            println!("tokens assembled, duration since start: {}", SystemTime::now().duration_since(time).unwrap().as_millis());
            tokens.0 = if let Ok(toks) =
                token_refactor::refactorer::refactor(tokens.0, tokens.1, &mut tokens.2)
            {
                tokens.1 = toks.1;
                toks.0
            } else {
                panic!("hruzostrasna pohroma");
            }; //tokenize(&string, true);
            println!(
                "time: {}",
                SystemTime::now().duration_since(time).unwrap().as_millis()
            );
            let parsed_tree = generate_tree(&tokens.0, &ast, &tokens.1);
            println!("Parsed.");
            println!(
                "time: {}",
                SystemTime::now().duration_since(time).unwrap().as_millis()
            );
            if false {
                if let Some(nodes) = &parsed_tree {
                    use tree_walker::tree_walker::ArgNodeType;
                    for nod in &nodes.nodes {
                        println!("{:?}", nod.0);
                        match nod.1 {
                            ArgNodeType::Array(arr) => {
                                for arg in arr {
                                    println!("{arg:?}");
                                }
                            }
                            ArgNodeType::Value(val) => {
                                println!("{val:?}");
                            }
                        }
                    }
                }
            }
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
            let mut string = string.into_bytes();
            use lexer::tokenizer::*;
            let tokens = tokenize(&string, true);
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
            } else {
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
            println!(
                "time: {}",
                SystemTime::now().duration_since(time).unwrap().as_millis()
            );
            black_box(vector);
        }
    }
}
