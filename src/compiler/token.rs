use std::fmt::{Display, Formatter};
use std::fmt::Result as FormatResult;

#[derive(Debug, Clone, Copy)]
pub enum Token<'a> {
    Name(TokenData<'a>),
    Arrow(TokenData<'a>),
    Or(TokenData<'a>),
    Endl(TokenData<'a>),
    Epsilon(TokenData<'a>),
    Literal(TokenData<'a>),
    Use(TokenData<'a>),
    PathSeperator(TokenData<'a>),
    OpenBrace(TokenData<'a>),
    CloseBrace(TokenData<'a>),
    QMark(TokenData<'a>),
}

impl<'a> Token<'a> {
    pub fn clean(self) -> Self {
        match self {
            Token::Literal(data) => {
                Token::Literal(TokenData::new(
                    &data.value[1..data.value.len() - 1],
                    data.line,
                    data.offset,
                ))
            }
            _ => self,
        }
    }
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
        match *self {
            Token::Name(ref data) => write!(f, "Name: {}", data),
            Token::Arrow(ref data) => write!(f, "Arrow: {}", data),
            Token::Or(ref data) => write!(f, "Or: {}", data),
            Token::Endl(ref data) => write!(f, "Endline: {}", data),
            Token::Epsilon(ref data) => write!(f, "Epsilon: {}", data),
            Token::Literal(ref data) => write!(f, "Literal: {}", data),
            Token::Use(ref data) => write!(f, "Use: {}", data),
            Token::PathSeperator(ref data) => write!(f, "Path Seperator: {}", data),
            Token::OpenBrace(ref data) => write!(f, "Open Brace: {}", data),
            Token::CloseBrace(ref data) => write!(f, "Close Brace: {}", data),
            Token::QMark(ref data) => write!(f, "Question Mark: {}", data),
        }
    }
}

#[derive(Debug, Clone, Default, Copy)]
pub struct TokenData<'a> {
    value: &'a [char],
    line: usize,
    offset: usize,
}

impl<'a> TokenData<'a> {
    pub fn new(value: &'a [char], line: usize, offset: usize) -> TokenData<'a> {
        TokenData {
            value: value,
            line: line,
            offset: offset,
        }
    }
}

impl<'a> Display for TokenData<'a> {
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
        write!(
            f,
            "value: '{}', line: {}, offset: {}",
            self.value.iter().collect::<String>(),
            self.line,
            self.offset
        )
    }
}