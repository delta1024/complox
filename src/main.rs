use std::{
    fmt::{self, Pointer},
    fs::File,
    io::Write,
    vec,
};

const PROGRAM_START: &str = "global _start\n_start:\n";
#[allow(dead_code)]
/// x86_64 Regesters.
enum Reg {
    /// a extended 64-bit regester
    Rax,
    /// 32-bit regester
    Eax,
    /// 16-bit regester
    Ax,
    /// high bit; bits 8-15
    Ah,
    // low bit; bits 0-7
    Al,
    /// b extended 64-bit regester
    Rbx,
    /// 32-bit regester
    Ebx,
    /// 16-bit regester
    Bx,
    /// high bit; bits 8-15
    Bh,
    // low bit; bits 0-7
    Bl,
    /// c extended 64-bit regester
    Rcx,
    /// 32-bit regester
    Ecx,
    /// 16-bit regester
    Cx,
    /// high bit; bits 8-15
    Ch,
    // low bit; bits 0-7
    Cl,
    /// d extended 64-bit regester
    Rdx,
    /// 32-bit regester
    Edx,
    /// 16-bit regester
    Dx,
    /// high bit; bits 8-15
    Dh,
    // low bit; bits 0-7
    Dl,
    /// regester base pointer (start of stack)
    Rbp,
    /// regester stack pointer (current location in stack, growing downwords)
    Rsp,
    /// regester source index (source for data copies)
    Rsi,
    /// regester destination index (destinataion for data copies)
    Rdi,
    /// 64-bit regester
    R8,
    /// 32-bit regester
    R8d,
    /// 16-bit regester
    R8w,
    /// low bit; bits 0-7.
    R8b,
    /// 64-bit regester
    R9,
    /// 32-bit regester
    R9d,
    /// 16-bit regester
    R9w,
    /// low bit; bits 0-7.
    R9b,
    /// 64-bit regester
    R10,
    /// 32-bit regester
    R10d,
    /// 16-bit regester
    R10w,
    /// low bit; bits 0-7.
    R10b,
    /// 64-bit regester
    R11,
    /// 32-bit regester
    R11d,
    /// 16-bit regester
    R11w,
    /// low bit; bits 0-7.
    R11b,
    /// 64-bit regester
    R12,
    /// 32-bit regester
    R12d,
    /// 16-bit regester
    R12w,
    /// low bit; bits 0-7.
    R12b,
    /// 64-bit regester
    R13,
    /// 32-bit regester
    R13d,
    /// 16-bit regester
    R13w,
    /// low bit; bits 0-7.
    R13b,
    /// 64-bit regester
    R14,
    /// 32-bit regester
    R14d,
    /// 16-bit regester
    R14w,
    /// low bit; bits 0-7.
    R14b,
    /// 64-bit regester
    R15,
    /// 32-bit regester
    R15d,
    /// 16-bit regester
    R15w,
    /// low bit; bits 0-7.
    R15b,
}
impl fmt::Display for Reg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rax => write!(f, "rax"),
            Self::Eax => write!(f, "eax"),
            Self::Ax => write!(f, "ax"),
            Self::Ah => write!(f, "ah"),
            Self::Al => write!(f, "al"),

            Self::Rbx => write!(f, "rbx"),
            Self::Ebx => write!(f, "ebx"),
            Self::Bx => write!(f, "bx"),
            Self::Bh => write!(f, "bh"),
            Self::Bl => write!(f, "bl"),

            Self::Rcx => write!(f, "rcx"),
            Self::Ecx => write!(f, "ecx"),
            Self::Cx => write!(f, "cx"),
            Self::Ch => write!(f, "ch"),
            Self::Cl => write!(f, "cl"),

            Self::Rdx => write!(f, "rdx"),
            Self::Edx => write!(f, "edx"),
            Self::Dx => write!(f, "dx"),
            Self::Dh => write!(f, "dh"),
            Self::Dl => write!(f, "dl"),

            Self::Rbp => write!(f, "rbp"),
            Self::Rsp => write!(f, "rsp"),
            Self::Rsi => write!(f, "rsi"),
            Self::Rdi => write!(f, "rdi"),

            Self::R8 => write!(f, "r8"),
            Self::R8d => write!(f, "r8d"),
            Self::R8w => write!(f, "r8w"),
            Self::R8b => write!(f, "r8b"),

            Self::R9 => write!(f, "r9"),
            Self::R9d => write!(f, "r9d"),
            Self::R9w => write!(f, "r9w"),
            Self::R9b => write!(f, "r9b"),

            Self::R10 => write!(f, "r10"),
            Self::R10d => write!(f, "r10d"),
            Self::R10w => write!(f, "r10w"),
            Self::R10b => write!(f, "r10b"),

            Self::R11 => write!(f, "r11"),
            Self::R11d => write!(f, "r11d"),
            Self::R11w => write!(f, "r11w"),
            Self::R11b => write!(f, "r11b"),

            Self::R12 => write!(f, "r12"),
            Self::R12d => write!(f, "r12d"),
            Self::R12w => write!(f, "r12w"),
            Self::R12b => write!(f, "r12b"),

            Self::R13 => write!(f, "r13"),
            Self::R13d => write!(f, "r13d"),
            Self::R13w => write!(f, "r13w"),
            Self::R13b => write!(f, "r13b"),

            Self::R14 => write!(f, "r14"),
            Self::R14d => write!(f, "r14d"),
            Self::R14w => write!(f, "r14w"),
            Self::R14b => write!(f, "r14b"),

            Self::R15 => write!(f, "r15"),
            Self::R15d => write!(f, "r15d"),
            Self::R15w => write!(f, "r15w"),
            Self::R15b => write!(f, "r15b"),
        }
    }
}
enum Directive {
    /// Eight bits
    Byte {
        regester: Reg,
        deref: bool,
        offset: Option<u32>,
    },
    /// Two Bytes,
    Word {
        regester: Reg,
        deref: bool,
        offset: Option<u32>,
    },
    /// Two words
    DWord {
        regester: Reg,
        deref: bool,
        offset: Option<u32>,
    },
    QWord {
        regester: Reg,
        deref: bool,
        offset: Option<u32>,
    },
}
impl fmt::Display for Directive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Byte {
                regester,
                deref,
                offset,
            } => {
                write!(f, "BYTE PTR ")?;
                match offset {
                    Some(offset) if *deref => write!(f, "[{regester}+{offset}]"),
                    Some(offset) => write!(f, "{regester}+{offset}"),
                    None if *deref => write!(f, "[{regester}]"),
                    None => write!(f, "{regester}"),
                }
            }
            Self::Word {
                regester,
                deref,
                offset,
            } => {
                write!(f, "WORD PTR ")?;
                match offset {
                    Some(offset) if *deref => write!(f, "[{regester}+{offset}]"),
                    Some(offset) => write!(f, "{regester}+{offset}"),
                    None if *deref => write!(f, "[{regester}]"),
                    None => write!(f, "{regester}"),
                }
            }
            Self::DWord {
                regester,
                deref,
                offset,
            } => {
                write!(f, "DWORD PTR ")?;
                match offset {
                    Some(offset) if *deref => write!(f, "[{regester}+{offset}]"),
                    Some(offset) => write!(f, "{regester}+{offset}"),
                    None if *deref => write!(f, "[{regester}]"),
                    None => write!(f, "{regester}"),
                }
            }
            Self::QWord {
                regester,
                deref,
                offset,
            } => {
                write!(f, "QWORD PTR ")?;
                match offset {
                    Some(offset) if *deref => write!(f, "[{regester}+{offset}]"),
                    Some(offset) => write!(f, "{regester}+{offset}"),
                    None if *deref => write!(f, "[{regester}]"),
                    None => write!(f, "{regester}"),
                }
            }
        }
    }
}
impl Directive {
    fn qword(regester: Reg, deref: bool, offset: Option<u32>) -> Directive {
        Self::QWord {
            regester,
            deref,
            offset,
        }
    }
    fn dword(regester: Reg, deref: bool, offset: Option<u32>) -> Directive {
        Self::DWord {
            regester,
            deref,
            offset,
        }
    }
    fn word(regester: Reg, deref: bool, offset: Option<u32>) -> Directive {
        Self::Word {
            regester,
            deref,
            offset,
        }
    }
    fn byte(regester: Reg, deref: bool, offset: Option<u32>) -> Directive {
        Self::Byte {
            regester,
            deref,
            offset,
        }
    }
}

enum Regester {
    Directive(Directive),
    Regester(Reg),
}
impl fmt::Display for Regester {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Regester::Regester(r) => r.fmt(f),
            Regester::Directive(d) => d.fmt(f),
        }
    }
}
impl From<Directive> for Regester {
    fn from(value: Directive) -> Self {
        Self::Directive(value)
    }
}

impl From<Reg> for Regester {
    fn from(value: Reg) -> Self {
        Self::Regester(value)
    }
}
enum Value {
    Number(u32),
    Regester(Regester),
}
impl<T: Into<Regester>> From<T> for Value {
    fn from(value: T) -> Self {
        Self::Regester(value.into())
    }
}
impl From<u32> for Value {
    fn from(value: u32) -> Self {
        Self::Number(value.into())
    }
}
impl From<Syscall> for Value {
    fn from(value: Syscall) -> Self {
        Self::Number(value.into())
    }
}
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(n) => n.fmt(f),
            Self::Regester(n) => n.fmt(f),
        }
    }
}

enum Syscall {
    Exit,
}
impl From<Syscall> for u32 {
    fn from(value: Syscall) -> Self {
        match value {
            Syscall::Exit => 60,
        }
    }
}
enum Instruction {
    Mov(Regester, Value),
    Push(Regester),
    Pop(Regester),
    Add(Regester, Regester),
    Sub(Regester, Regester),
    Mul(Regester, Regester),
    Xor(Regester, Regester),
    Syscall,
}

impl Instruction {
    fn exit<T: Into<Value>>(exit_code: T) -> Blob {
        Blob(vec![
            Self::Mov(Reg::Rdi.into(), exit_code.into()),
            Self::Mov(Reg::Rax.into(), Syscall::Exit.into()),
            Self::Syscall,
        ])
    }
    fn constant(cons: u32) -> Blob {
        Blob(vec![Self::Mov(Reg::Rax.into(), cons.into()), Self::Push(Reg::Rax.into())])
    }
    fn add() -> Blob {
        Blob(vec![
            Self::Pop(Reg::Rbx.into()),
            Self::Pop(Reg::Rax.into()),
            Self::Add(Reg::Rax.into(), Reg::Rbx.into()),
            Self::Push(Reg::Rax.into()),
        ])
    }
    fn sub() -> Blob {
        Blob(vec![
            Self::Pop(Reg::Rbx.into()),
            Self::Pop(Reg::Rax.into()),
            Self::Sub(Reg::Rax.into(), Reg::Rbx.into()),
            Self::Push(Reg::Rax.into()),
        ])
    }
    fn mul() -> Blob {
        Blob(vec![
            Self::Pop(Reg::Rbx.into()),
            Self::Pop(Reg::Rax.into()),
            Self::Mul(Reg::Rax.into(), Reg::Rbx.into()),
            Self::Push(Reg::Rax.into()),
        ])
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mov(d, s) => write!(f, "mov {d},{s}"),
            Self::Push(s) => write!(f, "push {s}"),
            Self::Pop(d) => write!(f, "pop {d}"),
            Self::Add(d, s) => write!(f, "add {d},{s}"),
            Self::Sub(d, s) => write!(f, "sub {d},{s}"),
            Self::Mul(d, s) => write!(f, "imul {d},{s}"),
            Self::Xor(d, s) => write!(f, "xor {d},{s}"),
            Self::Syscall => write!(f, "syscall"),
        }
    }
}

#[repr(transparent)]
struct Blob(Vec<Instruction>);
impl<T: Into<Vec<Instruction>>> From<T> for Blob {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}
impl fmt::Display for Blob {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for instruction in self {
            writeln!(f, "    {instruction}")?;
        }
        Ok(())
    }
}
impl<'a> IntoIterator for &'a Blob
where
    Self: 'a,
{
    type Item = &'a Instruction;
    type IntoIter = std::slice::Iter<'a, Instruction>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
impl IntoIterator for Blob {
    type Item = Instruction;
    type IntoIter = std::vec::IntoIter<Instruction>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
fn main() -> std::io::Result<()> {
    let mut file = File::create("out.asm")?;
    write!(file, "section .text\n{PROGRAM_START}")?;
    let program = vec![
        Instruction::constant(6),
        Instruction::constant(2),
        Instruction::sub(),
        Blob(vec![Instruction::Pop(Reg::Rax.into())]),
        Instruction::exit(Reg::Rax),
    ];
    for blob in program {
        writeln!(file, "{blob}")?;
    }

    Ok(())
}
