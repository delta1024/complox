use std::str::FromStr;

use super::{
    scanner::{Scanner, Token, TokenType},
    BinaryExpr, Error, Expression, UnaryExpr,
};

pub(crate) struct Parser<'a> {
    scanner: Scanner<'a>,
}

impl<'a> Parser<'a>
where
    Self: 'a,
{
    pub(crate) fn new(source: &'a str) -> Result<Self, Error> {
        Ok(Parser {
            scanner: Scanner::new(source),
        })
    }
    fn expression(&mut self) -> Result<Expression, Error> {
        self.equality()
    }
    fn equality(&mut self) -> Result<Expression, Error> {
        let mut expr = self.comparison()?;
        while let Some(r) = self.scanner.clone().peekable().peek().map(|r| {
            r.as_ref()
                .map(|t| t.id == TokenType::BangEqual || t.id == TokenType::EqualEqual)
        }) {
            if match r {
                Ok(b) => !b,
                Err(err) => return Err(err.clone()),
            } {
                break;
            }
            let operator = match self.scanner.next().unwrap()?.id {
                TokenType::EqualEqual => "==",
                TokenType::BangEqual => "!=",
                _ => unreachable!(),
            };
            let right = self.comparison()?;
            expr = BinaryExpr::new(expr, operator, right).into();
        }
        Ok(expr)
    }
    fn comparison(&mut self) -> Result<Expression, Error> {
        let mut expr = self.term()?;
        while let Some(r) = self.scanner.clone().peekable().peek().map(|r| {
            r.as_ref().map(|t| match t.id {
                TokenType::Greater
                | TokenType::GreaterEqual
                | TokenType::Less
                | TokenType::LessEqual => true,
                _ => false,
            })
        }) {
            if match r {
                Ok(b) => !b,
                Err(err) => return Err(err.clone()),
            } {
                break;
            }
            let operator = match self.scanner.next().unwrap()?.id {
                TokenType::GreaterEqual => ">=",
                TokenType::Greater => ">",
                TokenType::LessEqual => "<=",
                TokenType::Less => "<",
                _ => unreachable!(),
            };
            let right = self.term()?;
            expr = BinaryExpr::new(expr, operator, right).into();
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expression, Error> {
        let mut expr = self.factor()?;

        while let Some(r) = self.scanner.clone().peekable().peek().map(|r| {
            r.as_ref().map(|t| match t.id {
                TokenType::Minus | TokenType::Plus => true,
                _ => false,
            })
        }) {
            if match r {
                Ok(b) => !b,
                Err(err) => return Err(err.clone()),
            } {
                break;
            }
            let operator = match self.scanner.next().unwrap()?.id {
                TokenType::Minus => "-",
                TokenType::Plus => "+",
                _ => unreachable!(),
            };
            let right = self.factor()?;
            expr = BinaryExpr::new(expr, operator, right).into();
        }
        Ok(expr)
    }
    fn factor(&mut self) -> Result<Expression, Error> {
        let mut expr = self.unary()?;
        while let Some(r) = self.scanner.clone().peekable().peek().map(|r| {
            r.as_ref().map(|t| match t.id {
                TokenType::Slash | TokenType::Star => true,
                _ => false,
            })
        }) {
            if match r {
                Ok(r) => !r,
                Err(err) => return Err(err.clone()),
            } {
                break;
            }
            let operator = match self.scanner.next().unwrap()?.id {
                TokenType::Slash => "/",
                TokenType::Star => "*",
                _ => unreachable!(),
            };
            let right = self.unary()?;
            expr = BinaryExpr::new(expr, operator, right).into();
        }
        Ok(expr)
    }
    fn unary(&mut self) -> Result<Expression, Error> {
        if let Some(r) = self.scanner.clone().peekable().peek().map(|r| {
            r.as_ref().map(|t| match t.id {
                TokenType::Bang | TokenType::Minus => true,
                _ => false,
            })
        }) {
            if match r {
                Ok(r) => r,
                Err(err) => return Err(err.clone()),
            } {
                let operator = match self.scanner.next().unwrap()?.id {
                    TokenType::Bang => "!",
                    TokenType::Minus => "-",
                    _ => unreachable!(),
                };
                let right = self.unary()?;
                return Ok(UnaryExpr::new(operator, right).into());
            }
        }
        self.primary()
    }
    fn primary(&mut self) -> Result<Expression, Error> {
        use super::LiteralExpr;
        let token = self.scanner.next().unwrap()?;
        match &token.id {
            TokenType::False => return Ok(LiteralExpr::False.into()),
            TokenType::True => return Ok(LiteralExpr::True.into()),
            TokenType::Nil => return Ok(LiteralExpr::Nil.into()),
            TokenType::Number => return Ok(LiteralExpr::Number(token.lexeme.into()).into()),
            TokenType::String => {
                return Ok(
                    LiteralExpr::String((&token.lexeme[1..token.lexeme.len() - 1]).into()).into(),
                )
            }
            TokenType::LeftParen => {
                let expr = self.expression()?;
                self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
                return Ok(Expression::Grouping(expr.into()));
            }
            _ => unreachable!(),
        }
    }
    fn consume(&mut self, id: TokenType, err_message: &str) -> Result<Token, Error> {
        let mut peek = self.scanner.clone().peekable();
        let token = peek.peek();
        if let Some(r) = token.as_ref().map(|r| r.as_ref().map(|t| t.id == id)) {
            if match r {
                Ok(r) => r,
                Err(err) => return Err(err.clone()),
            } {
                return self.scanner.next().unwrap();
            } else {
                return Err(Error::new(
                    err_message,
                    token.unwrap().as_ref().unwrap().lexeme,
                    token.as_ref().unwrap().as_ref().unwrap().line,
                ));
            }
        } else {
            return Err(Error::new(err_message, " at end", self.scanner.line));
        }
    }
}

impl FromStr for Expression {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parser = Parser::new(s)?;
        parser.expression()
    }
}
