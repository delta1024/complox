use std::{fs::File, io::Write, vec};

mod x86_64;

use x86_64::{Program,Blob, OpCode, Reg, Regester, Section, Syscall};

fn main() -> std::io::Result<()> {
    let mut file = File::create("out.asm")?;
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

    let program = Program::new(None, text);
    write!(file, "{program}")?;

    Ok(())
}
