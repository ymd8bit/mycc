extern crate clap;

mod ast;
mod codegen;
// mod ir;
mod lexer;
mod parser;
mod token;
mod utils;

use clap::{App, Arg};
use codegen::x86::Codegen;
use lexer::Lexer;
use parser::Parser;
use std::path::Path;
use std::process::Command;

fn main() -> std::io::Result<()> {
    let matches = App::new("mycc")
        .version("0.1.0")
        .author("tkclimb")
        .about("mycc (MY C Compiler)")
        .arg(Arg::with_name("source_file").required(true))
        .get_matches();

    let source_file_path = matches
        .value_of("source_file")
        .expect("source file missing...");

    println!("source file path: {}", source_file_path);

    let contents = std::fs::read_to_string(source_file_path).expect("[error] read_to_string");
    let mut lexer = Lexer::new(contents.chars().collect());
    let token_list = lexer.tokenize();
    println!("{}", token_list);
    let mut parser = Parser::new(token_list);
    let module = parser.parse();
    println!("{}", module);

    let tmp_dir = Path::new("tmp/").canonicalize().unwrap();
    let tmp_asm_path = tmp_dir.join("tmp.s");
    let tmp_elf_path = tmp_dir.join("tmp.elf");

    let mut gen = Codegen::new();
    gen.export(&tmp_asm_path, module);

    let _ = Command::new("gcc")
        .arg("-o")
        .arg(&tmp_elf_path)
        .arg(&tmp_asm_path)
        .output()
        .expect("[Error] failed in the compilation...");
    let status = Command::new(&tmp_elf_path)
        .status()
        .expect("[Error] failed to run elf...");
    println!("[Result] {}", status);

    Ok(())
}
