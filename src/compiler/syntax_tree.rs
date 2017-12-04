#![allow(dead_code)]

use super::parser::SyntaxResult;
use super::lexer::LexicalResult;

pub type Recursive<T> = Box<Option<T>>;


#[derive(Debug)]
pub enum Grammar<'a> {
    ProdDecl(SyntaxResult<Prod<'a>>, Recursive<SyntaxResult<Grammar<'a>>>),
    PathDecl(SyntaxResult<Path<'a>>, Recursive<SyntaxResult<Grammar<'a>>>),
}

#[derive(Debug)]
pub struct Prod<'a> {
    name: LexicalResult<'a>,
    nullable: SyntaxResult<ONullable<'a>>,
    arrow: LexicalResult<'a>,
    union: SyntaxResult<Union<'a>>,
    endl: LexicalResult<'a>,
}

impl<'a> Prod<'a> {
    pub fn new(
        name: LexicalResult<'a>,
        nullable: SyntaxResult<ONullable<'a>>,
        arrow: LexicalResult<'a>,
        union: SyntaxResult<Union<'a>>,
        endl: LexicalResult<'a>,
    ) -> Self {
        Prod {
            name: name,
            nullable: nullable,
            arrow: arrow,
            union: union,
            endl: endl,
        }
    }
}

#[derive(Debug)]
pub struct Union<'a> {
    body: SyntaxResult<Body<'a>>,
    obody: Recursive<SyntaxResult<OBody<'a>>>,
}

impl<'a> Union<'a> {
    pub fn new(body: SyntaxResult<Body<'a>>, obody: Recursive<SyntaxResult<OBody<'a>>>) -> Self {
        Union {
            body: body,
            obody: obody,
        }
    }
}

#[derive(Debug)]
pub struct OBody<'a> {
    or: LexicalResult<'a>,
    union: SyntaxResult<Union<'a>>,
}

impl<'a> OBody<'a> {
    pub fn new(or: LexicalResult<'a>, union: SyntaxResult<Union<'a>>) -> Self {
        OBody {
            or: or,
            union: union,
        }
    }
}

#[derive(Debug)]
pub struct Body<'a> {
    part: SyntaxResult<Part<'a>>,
    nullable: SyntaxResult<ONullable<'a>>,
    rbody: Recursive<SyntaxResult<Body<'a>>>,
}

impl<'a> Body<'a> {
    pub fn new(
        part: SyntaxResult<Part<'a>>,
        nullable: SyntaxResult<ONullable<'a>>,
        rbody: Recursive<SyntaxResult<Body<'a>>>,
    ) -> Self {
        Body {
            part: part,
            nullable: nullable,
            rbody: rbody,
        }
    }
}

#[derive(Debug)]
pub enum Part<'a> {
    Literal(LexicalResult<'a>),
    LexicalRuleName(LexicalResult<'a>, LexicalResult<'a>, LexicalResult<'a>),
    Name(LexicalResult<'a>),
    Epsilon(LexicalResult<'a>),
}

#[derive(Debug)]
pub struct Nullable<'a> {
    qmark: LexicalResult<'a>,
}

impl<'a> Nullable<'a> {
    pub fn new(qmark: LexicalResult<'a>) -> Self {
        Nullable { qmark: qmark }
    }
}

pub type ONullable<'a> = Option<SyntaxResult<Nullable<'a>>>;


#[derive(Debug)]
pub struct Path<'a> {
    kuse: LexicalResult<'a>,
    name: LexicalResult<'a>,
    list: Recursive<SyntaxResult<PathItemList<'a>>>,
    endl: LexicalResult<'a>,
}

impl<'a> Path<'a> {
    pub fn new(
        kuse: LexicalResult<'a>,
        name: LexicalResult<'a>,
        list: Recursive<SyntaxResult<PathItemList<'a>>>,
        endl: LexicalResult<'a>,
    ) -> Self {
        Path {
            kuse: kuse,
            name: name,
            list: list,
            endl: endl,
        }
    }
}

#[derive(Debug)]
pub struct PathItemList<'a> {
    pathsep: LexicalResult<'a>,
    name: LexicalResult<'a>,
    list: Recursive<SyntaxResult<PathItemList<'a>>>,
}

impl<'a> PathItemList<'a> {
    pub fn new(
        pathsep: LexicalResult<'a>,
        name: LexicalResult<'a>,
        list: Recursive<SyntaxResult<PathItemList<'a>>>,
    ) -> Self {
        PathItemList {
            pathsep: pathsep,
            name: name,
            list: list,
        }
    }
}
