use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub struct Error {
    line: usize,
    location: String,
    message: String,
}

impl Error {
    fn new<T: ToString>(message: T, location: &str, line: usize) -> Self {
        Self {
            message: message.to_string(),
            location: location.to_string(),
            line,
        }
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[line {}] Error {}: {}",
            self.line, self.location, self.message
        )
    }
}
pub(crate) mod scanner;
//pub(crate) mod parser;

pub(crate) enum LiteralExpr {
    Number(Box<str>),
    String(Box<str>),
    True,
    False,
    Nil,
}
impl Display for LiteralExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nil => "nil".fmt(f),
            Self::False => "false".fmt(f),
            Self::True => "true".fmt(f),
            Self::Number(n) => write!(f, "{n}"),
            Self::String(s) => write!(f, "{s}"),
        }
    }
}
pub(crate) struct UnaryExpr {
    operator: &'static str,
    expression: Box<Expression>,
}
impl Display for UnaryExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {})", self.operator, self.expression)
    }
}
impl UnaryExpr {
    pub(crate) fn new<T: Into<Expression>>(operator: &'static str, expression: T) -> Self {
        let expression = Box::new(expression.into());
        Self { operator, expression }
    }
}
pub(crate) struct BinaryExpr {
    lhs: Box<Expression>,
    operator: &'static str,
    rhs: Box<Expression>,
}
impl Display for BinaryExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
write!(f, "({} {} {})", self.operator, self.lhs, self.rhs)
    }
}
impl<'a> BinaryExpr {
    pub(crate) fn new<T: Into<Expression>, U: Into<Expression>>(lhs: T, operator: &'static str, rhs: U) -> Self {
        let lhs = Box::new(lhs.into());
        let rhs = Box::new(rhs.into());
        Self { lhs, operator, rhs }
    }
}
pub(crate) enum Expression {
    Literal(LiteralExpr),
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    Grouping(Box<Expression>),
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(l) => l.fmt(f),
            Self::Unary(un) => un.fmt(f),
            Self::Binary(bin) => bin.fmt(f),
            Self::Grouping(group) => write!(f, "(group {group})"),
        }
    }
}
impl From<LiteralExpr> for Expression {
    fn from(arg: LiteralExpr) -> Self {
        Self::Literal(arg)
    }
}
impl<'a> From<BinaryExpr> for Expression {
    fn from(value: BinaryExpr) -> Self {
        Self::Binary(value)
    }
}
impl<'a> From<UnaryExpr> for Expression {
    fn from(value: UnaryExpr) -> Self {
        Self::Unary(value)
    }
}
