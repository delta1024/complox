use super::{Reg, Regester, Syscall, Value};
use crate::Blob;
use std::fmt;
pub(crate) enum OpCode {
    Mov(Regester, Value),
    Push(Regester),
    Pop(Regester),
    Add(Regester, Value),
    Sub(Regester, Value),
    Mul(Regester, Value),
    Div(Reg),
    Xor(Regester, Regester),
    Syscall,
}

impl OpCode {
    pub(crate) fn exit<T: Into<Value>>(exit_code: T) -> Blob {
        let exit_code = exit_code.into();
        let mov = if let Value::Number(0) = exit_code {
            Self::Xor(Reg::Rdi.into(), Reg::Rdi.into())
        } else {
            Self::Mov(Reg::Rdi.into(), exit_code)
        };
        Blob::from(vec![
            mov,
            Self::Mov(Reg::Rax.into(), Syscall::Exit.into()),
            Self::Syscall,
        ])
    }
    pub(crate) fn constant(cons: u32) -> Blob {
        Blob::from(vec![
            Self::Mov(Reg::Rax.into(), cons.into()),
            Self::Push(Reg::Rax.into()),
        ])
    }
    pub(crate) fn add() -> Blob {
        Blob::from(vec![
            Self::Pop(Reg::Rbx.into()),
            Self::Pop(Reg::Rax.into()),
            Self::Add(Reg::Rax.into(), Reg::Rbx.into()),
            Self::Push(Reg::Rax.into()),
        ])
    }
    pub(crate) fn add_v(a: u32, b: u32) -> Blob {
        Blob::from(vec![
            Self::Mov(Reg::Rax.into(), a.into()),
            Self::Mov(Reg::Rbx.into(), b.into()),
            Self::Sub(Reg::Rax.into(), Reg::Rbx.into()),
        ])
    }
    pub(crate) fn sub() -> Blob {
        Blob::from(vec![
            Self::Pop(Reg::Rbx.into()),
            Self::Pop(Reg::Rax.into()),
            Self::Sub(Reg::Rax.into(), Reg::Rbx.into()),
            Self::Push(Reg::Rax.into()),
        ])
    }
    pub(crate) fn sub_v(a: u32, b: u32) -> Blob {
        Blob::from(vec![
            Self::Mov(Reg::Rax.into(), a.into()),
            Self::Mov(Reg::Rbx.into(), b.into()),
            Self::Sub(Reg::Rax.into(), Reg::Rbx.into()),
        ])
    }
    pub(crate) fn mul() -> Blob {
        Blob::from(vec![
            Self::Pop(Reg::Rbx.into()),
            Self::Pop(Reg::Rax.into()),
            Self::Mul(Reg::Rax.into(), Reg::Rbx.into()),
            Self::Push(Reg::Rax.into()),
        ])
    }
    pub(crate) fn mul_v(a: u32, b: u32) -> Blob {
        Blob::from(vec![
            Self::Mov(Reg::Rax.into(), a.into()),
            Self::Mov(Reg::Rbx.into(), b.into()),
            Self::Mul(Reg::Rax.into(), Reg::Rbx.into()),
        ])
    }
    pub(crate) fn div() -> Blob {
        Blob::from(vec![
            OpCode::Pop(Reg::Rax.into()),                  // Move dividend into rax
            OpCode::Pop(Reg::Rbx.into()),                  // Move divisor into rbx
            OpCode::Xor(Reg::Rdx.into(), Reg::Rdx.into()), // Clear rdx
            OpCode::Div(Reg::Rbx),
            OpCode::Push(Reg::Rax.into()),
        ])
    }
    pub(crate) fn div_v(a: u32, b: u32) -> Blob {
        Blob::from(vec![
            OpCode::Mov(Reg::Rax.into(), b.into()), // Move dividend (bigger number) into rax
            OpCode::Mov(Reg::Rbx.into(), a.into()), // Move divisor (smaller number) into rbx
            OpCode::Xor(Reg::Rdx.into(), Reg::Rdx.into()), // Clear rdx
            OpCode::Div(Reg::Rbx),
        ])
    }
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mov(d, s) => write!(f, "mov {d},{s}"),
            Self::Push(s) => write!(f, "push {s}"),
            Self::Pop(d) => write!(f, "pop {d}"),
            Self::Add(d, s) => write!(f, "add {d},{s}"),
            Self::Sub(d, s) => write!(f, "sub {d},{s}"),
            Self::Mul(d, s) => write!(f, "imul {d},{s}"),
            Self::Div(s) => write!(f, "div {s}"),
            Self::Xor(d, s) => write!(f, "xor {d},{s}"),
            Self::Syscall => write!(f, "syscall"),
        }
    }
}
