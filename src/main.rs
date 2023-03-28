use std::{
    fs::{self, File},
    io::{Read, Write},
    process::{Command, Stdio},
};

mod ast;
mod ir;
mod x86_64;
use ir::Program;
const LOX_COMP_BUF: &'static str = "/tmp/out";

use ast::{scanner::Scanner, BinaryExpr, Expression, LiteralExpr, UnaryExpr};
fn compile_program(program: Program) -> std::io::Result<()> {
    let asm_path = format!("{LOX_COMP_BUF}.asm");
    let obj_path = format!("{LOX_COMP_BUF}.o");
    let mut file = File::create(&asm_path)?;
    write!(file, "{program}")?;

    Command::new("nasm")
        .args(&["-f", "elf64", &asm_path])
        .stderr(Stdio::inherit())
        .output()?;
    Command::new("ld")
        .arg(&obj_path)
        .stdout(Stdio::inherit())
        .output()?;
    fs::remove_file(asm_path)?;
    fs::remove_file(obj_path)?;
    Ok(())
}
fn run<'a>(input: &'a str) -> Result<(), ast::Error> {
    let scanner = Scanner::new(input);
    for token in scanner {
        match token {
            Ok(token) => println!("{token}"),
            Err(err) => eprintln!("{err}"),
        }
    }
    Ok(())
}
fn run_file(file: &str) -> std::io::Result<Option<Program>> {
    let mut file = File::open(file)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    match run(&input) {
        Ok(()) => (),
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(65);
        }
    }
    Ok(None)
}
fn run_repl() -> std::io::Result<()> {
    let mut input = String::new();
    loop {
        print!("> ");
        std::io::stdout().flush()?;
        if std::io::stdin().read_line(&mut input)? == 0 {
            break Ok(());
        }
        match run(&input) {
            Ok(()) => {
                input.clear();
            }
            Err(err) => {
                eprintln!("{err}");
                input.clear();
            }
        }
    }
}
fn main() -> std::io::Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() > 1 {
        println!("Usage: complox [file]");
    } else if args.len() == 0 {
        if let Some(program) = run_file(&args[1])? {
            compile_program(program)?;
        }
    } else {
        run_repl()?;
    }
    //   let expr = BinaryExpr::new(
    //       UnaryExpr::new("-",LiteralExpr::Number("123".to_string().into_boxed_str())),
    //       "*",
    //       Expression::Grouping(Box::new(Expression::Literal(LiteralExpr::Number("45.67".to_string().into_boxed_str())))),
    //   );
    //   println!("{expr}");
    Ok(())
}
