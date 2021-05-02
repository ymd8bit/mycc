#[macro_use]
extern crate clap;

mod lexer;

use lexer::Lexer;
use std::env;
// use std::fs::File;
// use std::io::{BufRead, BufReader};

use clap::{App, Arg};

fn main() -> std::io::Result<()> {
    let matches = App::new("mycc")
        .version("0.1.0")
        .author("tkclimb")
        .about("My C Compiler")
        .arg(Arg::with_name("source_file").required(true))
        .get_matches();

    let source_file_path = matches
        .value_of("source_file")
        .expect("source file missing...");

    println!("source file path: {}", source_file_path);

    let contents = std::fs::read_to_string(source_file_path).expect("[error] read_to_string");
    let mut lexer = Lexer::new(contents.chars().collect());
    let tokens = lexer.tokenize();
    println!("{:#?}", tokens);
    // let stmt_list = ModuleParser::new().parse(&contents.to_string()).unwrap();
    // println!("{:?}", stmt_list);

    Ok(())
}
