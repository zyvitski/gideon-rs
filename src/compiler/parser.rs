#![allow(dead_code)]
//See gideon.gideon for language grammar
use super::lexer::*;
use super::syntax_tree::*;
use super::token::*;
use super::frontend_error::*;

use std::cell::Cell;

pub type SyntaxResult<T> = Result<T, FrontendError>;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    cache: Cell<Option<LexicalResult<'a>>>,
}
impl<'a> Parser<'a> {
    pub fn new(input: &'a [char]) -> Self {
        Parser {
            lexer: Lexer::new(input),
            cache: Cell::new(None),
        }
    }

    pub fn parse(&'a self) -> SyntaxResult<Grammar<'a>> {
        let next = self.next();
        self.parse_grammar(next)
    }

    fn next(&'a self) -> LexicalResult<'a> {
        if let Some(cached) = self.cache.get() {
            self.cache.set(None);
            cached
        } else {
            self.lexer.next()
        }
    }

    fn cache_last(&'a self) {
        self.cache.set(Some(self.lexer.current_out()));
    }

    //Grammar -> Prod Grammar? | Path Grammar? ;
    fn parse_grammar(&'a self, current: LexicalResult<'a>) -> SyntaxResult<Grammar<'a>> {
        match current? {
            Token::Name(data) => {
                Ok(Grammar::ProdDecl(
                    self.parse_production(Ok(Token::Name(data))),
                    self.parse_rgrammar(self.next()),
                ))
            }
            Token::Use(data) => {
                Ok(Grammar::PathDecl(
                    self.parse_path(Ok(Token::Use(data))),
                    self.parse_rgrammar(self.next()),
                ))
            }
            _ => Err(FrontendError::ExpectedProdStartOrUse),
        }
    }
    //helper for above to break type recursion and allow nullability
    fn parse_rgrammar(
        &'a self,
        current: LexicalResult<'a>,
    ) -> Recursive<SyntaxResult<Grammar<'a>>> {
        match current {
            Ok(data) => match data {
                Token::Name(_) | Token::Use(_) => {
                    self.cache_last();
                    Box::new(Some(self.parse_grammar(self.next())))
                },
                _ => {
                    Box::new(None)
                }
            },
            _ => Box::new(None)
        }
    }

    //Prod -> {NAME} Nullable "->" Union ";" ;
    fn parse_production(&'a self, current: LexicalResult<'a>) -> SyntaxResult<Prod<'a>> {
        let name = match current? {
            Token::Name(data) => Ok(Token::Name(data)),
            _ => Err(FrontendError::ExpectedName),
        };

        let nullable = self.parse_nullable(self.next());

        let arrow = match self.next()? {
            Token::Arrow(data) => Ok(Token::Arrow(data)),
            _ => Err(FrontendError::ExpectedArrow),
        };

        let u = self.parse_union(self.next());

        let endl = match self.next()? {
            Token::Endl(data) => Ok(Token::Endl(data)),
            _ => Err(FrontendError::ExpectedEndl),
        };

        Ok(Prod::new(name, nullable, arrow, u, endl))
    }

    //Nullable -> "?" ? ;
    fn parse_nullable(&'a self, current: LexicalResult<'a>) -> SyntaxResult<ONullable<'a>> {
        match current? {
            Token::QMark(data) => {
                let out = Some(Ok(Nullable::new(Ok(Token::QMark(data)))));
                Ok(out)
            }
            _ => {
                self.cache_last();
                Ok(None)
            }, 
        }
    }

    //Union -> Body OBody ;
    fn parse_union(&'a self, current: LexicalResult<'a>) -> SyntaxResult<Union<'a>> {
        let body = self.parse_body(current);
        let obody = self.parse_obody(self.next());
        Ok(Union::new(body, obody))
    }

    //Body -> Part Nullable Body? ;
    fn parse_body(&'a self, current: LexicalResult<'a>) -> SyntaxResult<Body<'a>> {
        let part = self.parse_part(current);
        let nullable = self.parse_nullable(self.next());
        let rbody = self.parse_rbody(self.next());
        Ok(Body::new(part, nullable, rbody))
    }

    fn parse_rbody(&'a self, current: LexicalResult<'a>) -> Recursive<SyntaxResult<Body<'a>>>{
        match current {
            Ok(token) => {
                match token {
                     Token::Literal(_) 
                     | Token::Name(_) 
                     | Token::Epsilon(_) 
                     | Token::OpenBrace(_) => {
                         self.cache_last();
                         Box::new(Some(self.parse_body(self.next())))
                     }
                     _ => {
                         self.cache_last();
                         Box::new(None)
                     }
                }
            }
            _ => {
                self.cache_last();
                Box::new(None)
            }
        }
    }

    //OBody? -> "|" Union ;
    fn parse_obody(&'a self, current: LexicalResult<'a>) -> Recursive<SyntaxResult<OBody<'a>>> {
        let or = match current {
            Ok(data) => match data {
                Token::Or(data) => Ok(Token::Or(data)),
                _ => {self.cache_last(); return Box::new(None)}
            },
            _ => {self.cache_last(); return Box::new(None)}
        };
        let union = self.parse_union(self.next());

        Box::new(Some(Ok(OBody::new(or, union))))
    }

    //Part -> {LITERAL} 
    //      | "{" {NAME} "}" 
    //      | {NAME} 
    //      | {EPSILON};  
    fn parse_part(&'a self, current: LexicalResult<'a>) -> SyntaxResult<Part<'a>> {
        match current? {
            Token::Literal(data) => Ok(Part::Literal(Ok(Token::Literal(data)))),
            Token::Name(data) => Ok(Part::Name(Ok(Token::Name(data)))),
            Token::Epsilon(data) => Ok(Part::Epsilon(Ok(Token::Epsilon(data)))),
            Token::OpenBrace(brace) => {
                let obrace = Ok(Token::OpenBrace(brace));
                let name = match self.next()? {
                    Token::Name(data) => Ok(Token::Name(data)),
                    _ => Err(FrontendError::ExpectedName)
                };
                let cbrace = match self.next()? {
                    Token::CloseBrace(brace) => Ok(Token::CloseBrace(brace)),
                    _=>Err(FrontendError::ExpectedCloseCurlyBrace)
                };
                Ok(Part::LexicalRuleName(obrace, name, cbrace))
            }
            _ => Err(FrontendError::ExpectedPart)
        }
    }



    //Path -> "use" {NAME} PathItemList ";" ;
    fn parse_path(&'a self, current: LexicalResult<'a>) -> SyntaxResult<Path<'a>> {
        let kuse = match current? {
            Token::Use(data) => Ok(Token::Use(data)),
            _ => Err(FrontendError::ExpectedUse),
        };
        let name = match self.next()? {
            Token::Name(data) => Ok(Token::Name(data)),
            _ => Err(FrontendError::ExpectedName),
        };
        let path_item_list = self.parse_path_item_list(self.next());
        if path_item_list.is_none() {
            self.cache_last();
        }
        let endl = match self.next()? {
            Token::Endl(data) => Ok(Token::Endl(data)),
            _ => Err(FrontendError::ExpectedEndl),
        };
        Ok(Path::new(kuse, name, path_item_list, endl))
    }


    //PathItemList? -> "::" {NAME} PathItemList ; 
    fn parse_path_item_list(
        &'a self,
        current: LexicalResult<'a>,
    ) -> Recursive<SyntaxResult<PathItemList<'a>>> {
        let path_seperator = match current {
            Ok(data) => {
                match data {
                    Token::PathSeperator(data) => Ok(Token::PathSeperator(data)),
                    _ => {
                        self.cache_last();
                        return Box::new(None)
                    },
                }
            }
            Err(_) => {
                self.cache_last();
                return Box::new(None)
            },
        };
        let name = match self.next() {
            Ok(data) => match data{
                Token::Name(data) => Ok(Token::Name(data)),
                _ => Err(FrontendError::ExpectedName),
            }
            Err(what) => {
                Err(what)
            }
        };

        let path_item_list = self.parse_path_item_list(self.next());

        Box::new(Some(Ok(PathItemList::new(path_seperator, name, path_item_list))))
    }
}
