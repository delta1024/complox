use super::Error;
use std::{fmt::Display, ops::RangeInclusive, str::CharIndices};
#[derive(Debug, Clone, Copy,PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}
impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
#[derive(Clone, Copy)]
pub(crate) struct Token<'a> {
    pub(crate) id: TokenType,
    pub(crate) lexeme: &'a str,
    pub(crate) line: usize,
}
impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.line, self.id, self.lexeme,)
    }
}
#[repr(transparent)]
#[derive(Clone, Debug)]
struct Pos((usize, usize));
impl Pos {
    fn new() -> Self {
        Self((0, 0))
    }
    fn sync(&mut self) {
        let (start, current) = &mut self.0;
        *start = *current;
    }
    fn cur_lexm(&self) -> RangeInclusive<usize> {
        self.0 .0..=self.0 .1
    }
    fn cur_pos(&self) -> usize {
        self.0 .1
    }
}
#[derive(Debug, Clone)]
pub(crate) struct Scanner<'a> {
    location: Pos,
    source: &'a str,
    pub(super) line: usize,
    chars: CharIndices<'a>,
}

impl<'a> Scanner<'a> {
    pub(crate) fn new(source: &'a str) -> Self {
        Self {
            location: Pos::new(),
            source,
            line: 1,
            chars: source.char_indices(),
        }
    }
    fn is_at_end(&self) -> bool {
        self.location.cur_pos() >= self.source.len()
    }
    fn advance(&mut self) -> Option<char> {
        let (indc, char) = self.chars.next()?;
        self.location.0 .1 = indc;
        Some(char)
    }
    fn make_token(&self, id: TokenType) -> Token<'a> {
        Token {
            id,
            lexeme: &self.source[self.location.cur_lexm()],
            line: self.line,
        }
    }
    fn string(&mut self) -> Option<Result<Token<'a>, Error>> {
        let mut chars = self.chars.clone().peekable();
        while chars.peek().map(|t| t.1) != Some('"') {
            if chars.next().map(|t| t.1) == Some('\n') {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Some(Err(Error::new("Unterminated string", "", self.line)));
        }

        // the closing '"'
        self.advance();

        Some(Ok(self.make_token(TokenType::String)))
    }
    fn number(&mut self) -> Option<Result<Token<'a>, Error>> {
        let mut chars = self.chars.clone().peekable();
        while match chars.peek() {
            Some(c) if c.1.is_ascii_digit() => true,
            _ => false,
        } {
            self.advance();
            chars.next();
        }

        if Some(true) == chars.peek().map(|c| c.1 == '.')
            && Some(true) == {
                let mut chars = chars.clone();
                chars.next();
                chars.peek().map(|c| c.1.is_ascii_digit())
            }
        {
            // consume the '.'
            chars.next();
            self.advance();

            while match chars.peek() {
                Some(c) if c.1.is_ascii_digit() => true,
                _ => false,
            } {
                self.advance();
                chars.next();
            }
        }
        Some(Ok(self.make_token(TokenType::Number)))
    }
}
impl<'a> Iterator for Scanner<'a>
where
    Self: 'a,
{
    type Item = Result<Token<'a>, Error>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_at_end() {
            return None;
        }
        while match self.chars.clone().peekable().peek().map(|t| t.1) {
            Some('\n') => {
                self.line += 1;
                true
            }
            Some(c) if c.is_ascii_whitespace() => true,
            _ => false,
        } {
            self.advance();
        }
        let c = self.advance()?;
        self.location.sync();
        match c {
            '(' => Some(Ok(self.make_token(TokenType::LeftParen))),
            ')' => Some(Ok(self.make_token(TokenType::RightParen))),
            '{' => Some(Ok(self.make_token(TokenType::LeftBrace))),
            '}' => Some(Ok(self.make_token(TokenType::RightBrace))),
            ',' => Some(Ok(self.make_token(TokenType::Comma))),
            '.' => Some(Ok(self.make_token(TokenType::Dot))),
            '-' => Some(Ok(self.make_token(TokenType::Minus))),
            '+' => Some(Ok(self.make_token(TokenType::Plus))),
            ';' => Some(Ok(self.make_token(TokenType::Semicolon))),
            '*' => Some(Ok(self.make_token(TokenType::Star))),
            '!' if self.chars.clone().peekable().peek().map(|tup| tup.1) == Some('=') => {
                // consume '!'
                self.advance();
                Some(Ok(self.make_token(TokenType::BangEqual)))
            }
            '!' => Some(Ok(self.make_token(TokenType::Bang))),
            '=' if self.chars.clone().peekable().peek().map(|tup| tup.1) == Some('=') => {
                // consume '='
                self.advance();
                Some(Ok(self.make_token(TokenType::EqualEqual)))
            }
            '=' => Some(Ok(self.make_token(TokenType::Equal))),
            '<' if self.chars.clone().peekable().peek().map(|tup| tup.1) == Some('=') => {
                // consume '<'
                self.advance();
                Some(Ok(self.make_token(TokenType::LessEqual)))
            }
            '<' => Some(Ok(self.make_token(TokenType::Less))),
            '>' if self.chars.clone().peekable().peek().map(|tup| tup.1) == Some('=') => {
                // consume '>'
                self.advance();
                Some(Ok(self.make_token(TokenType::GreaterEqual)))
            }
            '>' => Some(Ok(self.make_token(TokenType::Greater))),
            '/' if self.chars.clone().peekable().peek().map(|tup| tup.1) == Some('/') => {
                let mut peek = self.chars.clone().peekable();
                // consume the second '/'
                self.advance();
                peek.next();
                while peek.peek().map(|tup| tup.1) != Some('\n') && !self.is_at_end() {
                    peek.next();
                    self.advance();
                }
                self.next()
            }
            '/' => Some(Ok(self.make_token(TokenType::Slash))),
            '"' => self.string(),
            _ if c.is_ascii_digit() => self.number(),
            _ if c.is_alphabetic() => match {
                while self
                    .chars
                    .clone()
                    .peekable()
                    .peek()
                    .map(|t| t.1.is_alphanumeric())
                    == Some(true)
                {
                    self.advance();
                }
                &self.source[self.location.cur_lexm()]
            } {
                "and" => Some(Ok(self.make_token(TokenType::And))),
                "class" => Some(Ok(self.make_token(TokenType::Class))),
                "else" => Some(Ok(self.make_token(TokenType::Else))),
                "false" => Some(Ok(self.make_token(TokenType::False))),
                "for" => Some(Ok(self.make_token(TokenType::For))),
                "fun" => Some(Ok(self.make_token(TokenType::Fun))),
                "if" => Some(Ok(self.make_token(TokenType::If))),
                "nil" => Some(Ok(self.make_token(TokenType::Nil))),
                "or" => Some(Ok(self.make_token(TokenType::Or))),
                "print" => Some(Ok(self.make_token(TokenType::Print))),
                "return" => Some(Ok(self.make_token(TokenType::Return))),
                "super" => Some(Ok(self.make_token(TokenType::Super))),
                "this" => Some(Ok(self.make_token(TokenType::This))),
                "true" => Some(Ok(self.make_token(TokenType::True))),
                "var" => Some(Ok(self.make_token(TokenType::Var))),
                "while" => Some(Ok(self.make_token(TokenType::While))),
                _ => Some(Ok(self.make_token(TokenType::Identifier))),
            },
            _ => Some(Err(Error::new(
                format!("Unexpected token: {} ", c),
                "",
                self.line,
            ))),
        }
    }
}
