use logos::Logos;
use rug::{Float, ops::CompleteRound};

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    Error,

    #[regex(r"[0-9]+(\.[0-9]+)?", |lex| Float::parse(lex.slice()).unwrap().complete(64))]
    Number(Float),

    #[token("+")]
    Add,
    #[token("-")]
    Sub,
    #[token("*")]
    Mul,
    #[token("/")]
    Div,

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
    #[token("pi")]
    Pi,
    #[token("e")]
    E,
    #[token("tau")]
    Tau,
    #[token("inf")]
    Inf,
    #[token("nan")]
    Nan,
    // functions
}
