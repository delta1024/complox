use std::fmt;
pub(crate) mod regesters;
pub(crate) mod directives;
pub(crate) mod op_code;
pub(crate) use op_code::*;
pub(crate) use directives::*;
pub(crate) use  regesters::*;
pub(crate) enum Value {
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

pub(crate) enum Syscall {
    /// 1: %rax(0x01), %rdi(unsigned int fd), %rsi(char *buf), %rdx(size_t count)
    Write,
    /// 60: rax(0x3c), rdi(int error_code)
    Exit,
}
impl From<Syscall> for u32 {
    fn from(value: Syscall) -> Self {
        match value {
            Syscall::Write => 0x01,
            Syscall::Exit => 0x3c,
        }
    }
}
