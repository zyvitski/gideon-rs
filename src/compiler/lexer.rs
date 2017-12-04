/*
Gideon lexical specification

Whitespace ignored

NAME:  ( \alphabetic | _ )+ ( \alphanumeric | _ )*
ARROW: ->
OR: \|
ENDL ;
EPSILON: ϵ | None
LEXICAL: \{ NAME \}
LITERAL: "([^"] |\\")+"
COMMENT: #.*\r?\n
*/

use super::token::*;
use super::frontend_error::*;
use std::cell::Cell;

const NONE: &[char; 4] = &['N', 'o', 'n', 'e'];
const USE: &[char; 3] = &['u', 's', 'e'];

pub type LexicalResult<'a> = Result<Token<'a>, FrontendError>;

pub struct Lexer<'a> {
    input: &'a [char],
    current: Cell<usize>,
    last: Cell<usize>,
    end: Cell<usize>,
    line: Cell<usize>,
    offset: Cell<usize>,
    current_token: Cell<LexicalResult<'a>>,
}
impl<'a> Lexer<'a> {
    pub fn new(input: &'a [char]) -> Lexer<'a> {
        Lexer {
            input: input,
            current: Cell::from(0),
            last: Cell::from(0),
            end: Cell::from(input.len()),
            line: Cell::from(1),
            offset: Cell::from(0),
            current_token: Cell::from(Err(FrontendError::Default)),
        }
    }
    fn current(&'a self) -> Option<char> {
        if self.current < self.end {
            Some(self.input[self.current.get()])
        } else {
            None
        }
    }
    fn step(&'a self) {
        self.current.set(self.current.get() + 1);
    }
    fn skip(&'a self) {
        self.step();
        self.last.set(self.last.get() + 1);
        self.offset.set(self.offset.get() + 1);
    }
    fn current_match(&'a self) -> &'a [char] {
        &self.input[self.last.get()..self.current.get()]
    }

    fn accept(&'a self) -> TokenData<'a> {
        let slice = self.current_match();
        let line_inc = slice.iter().fold(0, |n: usize, c: &char| if *c == '\n' {
            n + 1
        } else {
            n
        });
        self.last.set(self.current.get());
        let out = TokenData::new(slice, self.line.get(), self.offset.get());
        if line_inc > 0 {
            self.line.set(self.line.get() + line_inc);
            self.offset.set(0);
        } else {
            self.offset.set(self.offset.get() + slice.len());
        }
        out
    }

    pub fn current_out(&'a self) -> LexicalResult<'a> {
        self.current_token.get()
    }

    pub fn next(&'a self) -> LexicalResult<'a> {
        if let Some(currc) = self.current() {
            let tok = match currc {
                //Match Or
                '|' => {
                    self.step();
                    Ok(Token::Or(self.accept()))
                }
                //Lexical Rule Name
                '{' => {
                    self.step();
                    Ok(Token::OpenBrace(self.accept()))
                }
                '}' => {
                    self.step();
                    Ok(Token::CloseBrace(self.accept()))
                }
                //Literal
                '"' => {
                    self.step();
                    while let Some(currc) = self.current() {
                        if currc == '"' {
                            self.step();
                            break;
                        } else if currc == '\\' {
                            self.step();
                            if let Some(currc) = self.current() {
                                match currc {
                                    '"' | '\\' => self.step(),
                                    _ => {
                                        self.current_token.set(Err(
                                            FrontendError::ExpectedEscapeSequence,
                                        ));
                                        return self.current_token.get();
                                    }
                                }
                            } else {
                                self.current_token.set(
                                    Err(FrontendError::ExpectedEscapeSequence),
                                );
                                return self.current_token.get();
                            }
                        } else {
                            self.step();
                        }
                    }
                    Ok(Token::Literal(self.accept()).clean())
                }
                //Epsilon
                'ϵ' => {
                    self.step();
                    Ok(Token::Epsilon(self.accept()))
                }
                //Arrow Start
                '-' => {
                    self.step();
                    if let Some(look) = self.current() {
                        if look == '>' {
                            self.step();
                            Ok(Token::Arrow(self.accept()))
                        } else {
                            self.current_token.set(Err(FrontendError::ExpectedArrowTip));
                            return self.current_token.get();
                        }
                    } else {
                        self.current_token.set(
                            Err(FrontendError::ExpectedMoreInput),
                        );
                        return self.current_token.get();
                    }
                }
                ';' => {
                    self.step();
                    Ok(Token::Endl(self.accept()))
                }
                ':' => {
                    self.step();
                    if let Some(look) = self.current() {
                        if look == ':' {
                            self.step();
                            Ok(Token::PathSeperator(self.accept()))
                        } else {
                            self.current_token.set(Err(FrontendError::ExpectedColon));
                            return self.current_token.get();
                        }
                    } else {
                        self.current_token.set(
                            Err(FrontendError::ExpectedMoreInput),
                        );
                        return self.current_token.get();
                    }
                }
                '#' => {
                    self.skip();
                    'comment: while let Some(currc) = self.current() {
                        if currc == '\n' {
                            self.skip();
                            self.line.set(self.line.get() + 1);
                            self.offset.set(0);
                            break 'comment;
                        } else {
                            self.skip();
                        }
                    }
                    self.current_token.set(self.next());
                    return self.current_token.get();
                }
                '?' => {
                    self.step();
                    Ok(Token::QMark(self.accept()))
                }
                _ => {
                    if currc.is_alphabetic() || currc == '_' {
                        //NAME
                        while let Some(currc) = self.current() {
                            if currc.is_alphabetic() || currc == '_' {
                                self.step();
                            } else {
                                break;
                            }
                        }
                        if (self.current.get() - self.last.get()) > 0 {
                            while let Some(currc) = self.current() {
                                if currc.is_alphanumeric() || currc == '_' {
                                    self.step();
                                } else {
                                    break;
                                }
                            }
                        } else {
                            self.current_token.set(
                                Err(FrontendError::ExpectedIdentifier),
                            );
                            return self.current_token.get();
                        }
                        if self.current_match() == NONE {
                            Ok(Token::Epsilon(self.accept()))
                        } else if self.current_match() == USE {
                            Ok(Token::Use(self.accept()))
                        } else {
                            Ok(Token::Name(self.accept()))
                        }
                    } else if currc.is_whitespace() {
                        self.skip();
                        //Skip all whitespace
                        'whitespace: while let Some(currc) = self.current() {
                            if currc.is_whitespace() {
                                self.skip();
                                if currc == '\n' {
                                    self.line.set(self.line.get() + 1);
                                    self.offset.set(0);
                                }
                            } else {
                                break 'whitespace;
                            }
                        }
                        self.current_token.set(self.next());
                        return self.current_token.get();
                    } else {
                        self.current_token.set(
                            Err(FrontendError::UnrecognizedInput),
                        );
                        return self.current_token.get();
                    }
                }
            };
            self.current_token.set(tok);
            self.current_token.get()
        } else {
            self.current_token.set(Err(FrontendError::EOI));
            self.current_token.get()
        }
    }
}