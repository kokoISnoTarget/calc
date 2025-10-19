use std::str::FromStr;

use logos::Logos;
use rug::{Float, ops::CompleteRound};

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    Error,

    #[regex(r"[0-9]+(\.[0-9]+)?", |lex| Float::parse(lex.slice()).unwrap().complete(64))]
    Number(Float),

    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,

    #[token("**")]
    Pow,
    #[token("%")]
    Mod,
    #[token("!")]
    Fact,

    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,

    // Constants
    #[regex(r"[a-zA-Z]+", |lex| Identifier::from_str(lex.slice()))]
    Identifier(Identifier),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Identifier {
    Constant(Constant),
    Function(Function),
}

impl FromStr for Identifier {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pi" => Ok(Identifier::Constant(Constant::Pi)),
            "e" => Ok(Identifier::Constant(Constant::E)),
            "tau" => Ok(Identifier::Constant(Constant::Tau)),
            "inf" => Ok(Identifier::Constant(Constant::Inf)),
            "nan" => Ok(Identifier::Constant(Constant::Nan)),
            "sin" => Ok(Identifier::Function(Function::Sin)),
            "cos" => Ok(Identifier::Function(Function::Cos)),
            "tan" => Ok(Identifier::Function(Function::Tan)),
            "exp" => Ok(Identifier::Function(Function::Exp)),
            "sqrt" => Ok(Identifier::Function(Function::Sqrt)),
            _ => Err(()),
        }
    }
}

#[derive(Logos, Clone, Debug, PartialEq)]
pub enum Constant {
    Pi,
    E,
    Tau,
    Inf,
    Nan,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Function {
    Sin,
    Cos,
    Tan,
    Exp,
    Sqrt,
}
