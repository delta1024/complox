use std::{
    fs::File,
    io::Write,
    process::{Command, Stdio},
    vec,
};

mod x86_64;
const LOX_COMP_BUF: &'static str = "/tmp/out";

use x86_64::{Blob, OpCode, Program, Reg, Regester, Section, Syscall};

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
        .arg(obj_path)
        .stdout(Stdio::inherit())
        .output()?;
    Ok(())
}
fn main() -> std::io::Result<()> {

    let text = Section::new(
        "_start",
        vec![
            OpCode::mul_v(2, 4),
            // Print the quotient
            Blob::from(vec![
                OpCode::Add(Reg::Al.into(), ('0' as u32).into()), // Convert quotient to ascii
                OpCode::Mov(Regester::Deref(Reg::Rsp), Reg::Al.into()), // store quotient in stack
                OpCode::Mov(Reg::Rax.into(), Syscall::Write.into()), // Set syscall
                OpCode::Mov(Reg::Rdi.into(), 1.into()),           // File descriptor (stdout)
                OpCode::Mov(Reg::Rsi.into(), Reg::Rsp.into()),    // Pointer to quotient string
                OpCode::Mov(Reg::Rdx.into(), 1.into()),           // lenght of string
                OpCode::Syscall,
            ]),
            // Exit program
            OpCode::exit(0),
        ],
    );

    let program = Program::new(None, vec![text]);
    compile_program(program)
}
