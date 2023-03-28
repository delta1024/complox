use super::Reg;
use std::fmt;
pub(crate) enum Directive {
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
                write!(f, "BYTE ")?;
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
                write!(f, "WORD ")?;
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
                write!(f, "DWORD ")?;
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
                write!(f, "QWORD ")?;
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
    pub(crate) fn qword(regester: Reg, deref: bool, offset: Option<u32>) -> Directive {
        Self::QWord {
            regester,
            deref,
            offset,
        }
    }
    pub(crate) fn dword(regester: Reg, deref: bool, offset: Option<u32>) -> Directive {
        Self::DWord {
            regester,
            deref,
            offset,
        }
    }
    pub(crate) fn word(regester: Reg, deref: bool, offset: Option<u32>) -> Directive {
        Self::Word {
            regester,
            deref,
            offset,
        }
    }
    pub(crate) fn byte(regester: Reg, deref: bool, offset: Option<u32>) -> Directive {
        Self::Byte {
            regester,
            deref,
            offset,
        }
    }
}
