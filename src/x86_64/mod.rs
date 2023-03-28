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
#[repr(transparent)]
pub(crate) struct Blob(Vec<OpCode>);
impl<T: Into<Vec<OpCode>>> From<T> for Blob {
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
    type Item = &'a OpCode;
    type IntoIter = std::slice::Iter<'a, OpCode>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
impl IntoIterator for Blob {
    type Item = OpCode;
    type IntoIter = std::vec::IntoIter<OpCode>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

pub(crate) struct Section {
    name: String,
    code: Vec<Blob>,
}
impl Section {
    pub(crate) fn new(name: &str, code: Vec<Blob>) -> Self {
        Self {
            name: name.to_string(),
            code,
        }
    }
}
impl fmt::Display for Section {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}:", &self.name)?;

        for blob in &self.code {
            writeln!(f, "{blob}")?;
        }
        Ok(())
    }
}
pub(crate) struct Program {
    data: Option<Section>,
    text: Vec<Section>,
}
impl Program {
    pub(crate) fn new(data: Option<Section>, text: Vec<Section>) -> Program {
        Program { data, text }
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(section) = &self.data {
            writeln!(f, "section .data")?;
            for glob in &section.code {
                writeln!(f, "{glob}")?;
            }
        }
        writeln!(f, "section .text")?;
        writeln!(f, "  global _start")?;
        for glob in &self.text {
            write!(f, "{glob}")?;
        }
        Ok(())
    }
}
