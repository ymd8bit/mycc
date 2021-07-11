extern crate clap;

mod ast;
mod codegen;
// mod ir;
mod lexer;
mod parser;
mod token;
mod utils;

use clap::{App, Arg};
use std::path::{Path, PathBuf};
use std::process::Command;

use codegen::x86::Codegen;
use lexer::Lexer;
use parser::Parser;

fn compile(source_file_path: &str, tmp_dir: &PathBuf) -> PathBuf {
    let contents = std::fs::read_to_string(source_file_path).expect("[error] read_to_string");
    let mut lexer = Lexer::new(contents.chars().collect());
    let token_list = lexer.tokenize();
    // println!("{}", token_list);
    let mut parser = Parser::new(token_list);
    let module = parser.parse();
    // println!("{}", module);
    let mut gen = Codegen::new();
    let tmp_asm_path = tmp_dir.join("tmp.s");
    gen.export(&tmp_asm_path, module);
    tmp_asm_path
}

fn main() -> std::io::Result<()> {
    let matches = App::new("mycc")
        .version("0.1.0")
        .author("tkclimb")
        .about("mycc (MY C Compiler)")
        .arg(Arg::with_name("source_files").required(true).min_values(1))
        .get_matches();

    let source_file_paths: Vec<&str> = matches
        .values_of("source_files")
        .expect("source file missing...")
        .collect();

    let source_file_path = source_file_paths[0];

    println!("source file path: {}", source_file_path);

    let tmp_dir = Path::new("tmp/").canonicalize().unwrap();
    // if let Err(err) = std::fs::create_dir(tmp_dir) {
    //     panic!(format!(
    //         "create directory {} failed at ",
    //         tmp_dir.to_str().unwrap()
    //     ))
    // };
    let tmp_elf_path = tmp_dir.join("tmp.elf");

    let mut cmd = Command::new("gcc");
    cmd.arg("-g").arg("-O0").arg("-o").arg(&tmp_elf_path);

    for source_file_path in source_file_paths {
        let extension = Path::new(source_file_path)
            .extension()
            .expect("file was given without extenstion...");
        if extension == "c" {
            let tmp_asm_path = compile(source_file_path, &tmp_dir);
            cmd.arg(&tmp_asm_path);
        } else {
            cmd.arg(&source_file_path);
        }
    }

    cmd.status().expect("[Error] failed in the compilation...");
    let status = Command::new(&tmp_elf_path)
        .status()
        .expect("[Error] failed to run elf...");
    println!("[Result] {}", status);

    Ok(())
}
