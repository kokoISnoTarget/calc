use chumsky::input::ValueInput;
use chumsky::pratt::{infix, left, postfix};
use chumsky::{
    Parser,
    pratt::{prefix, right},
    prelude::{recursive, *},
};
use rug::{
    Float,
    float::{Constant, Special},
};

use crate::token::Token;

pub enum Ast {
    Number(Float),
    BinOp(BinaryOperation, Box<Ast>, Box<Ast>),
    UnaryOp(UnaryOperation, Box<Ast>),
    Paren(Box<Ast>),
}

pub enum BinaryOperation {
    Add,
    Sub,
    Mul,
    Div,

    Pow,
    Mod,
}

pub enum UnaryOperation {
    Negative,
    Positive,
    Fact,
}

pub fn parser<'tokens, 'src: 'tokens, I>()
-> impl Parser<'tokens, I, Ast, extra::Err<Rich<'tokens, Token>>>
where
    I: ValueInput<'tokens, Token = Token, Span = SimpleSpan>,
{
    recursive(|expr| {
        let num = select! {
            Token::Number(num) => Ast::Number(num),
            Token::Pi => Ast::Number(Float::with_val(64, Constant::Pi)),
            Token::Inf => Ast::Number(Float::with_val(64, Special::Infinity)),
            Token::Nan => Ast::Number(Float::with_val(64, Special::Nan)),
        };

        let paren = expr
            .clone()
            .delimited_by(just(Token::LeftParen), just(Token::RightParen))
            .map(|e| Ast::Paren(Box::new(e)));

        let atom = num.or(paren);

        atom.pratt((
            // Prefix unary operators
            prefix(3, just(Token::Add), |_, x, _| {
                Ast::UnaryOp(UnaryOperation::Positive, Box::new(x))
            }),
            prefix(3, just(Token::Sub), |_, x, _| {
                Ast::UnaryOp(UnaryOperation::Negative, Box::new(x))
            }),
            // Postfix unary operator: factorial
            postfix(4, just(Token::Fact), |x, _, _| {
                Ast::UnaryOp(UnaryOperation::Fact, Box::new(x))
            }),
            // Binary operators
            infix(right(5), just(Token::Pow), |x, _, y, _| {
                Ast::BinOp(BinaryOperation::Pow, Box::new(x), Box::new(y))
            }),
            infix(left(2), just(Token::Mul), |x, _, y, _| {
                Ast::BinOp(BinaryOperation::Mul, Box::new(x), Box::new(y))
            }),
            infix(left(2), just(Token::Div), |x, _, y, _| {
                Ast::BinOp(BinaryOperation::Div, Box::new(x), Box::new(y))
            }),
            infix(left(2), just(Token::Mod), |x, _, y, _| {
                Ast::BinOp(BinaryOperation::Mod, Box::new(x), Box::new(y))
            }),
            infix(left(2), just(Token::Add), |x, _, y, _| {
                Ast::BinOp(BinaryOperation::Add, Box::new(x), Box::new(y))
            }),
            infix(left(1), just(Token::Sub), |x, _, y, _| {
                Ast::BinOp(BinaryOperation::Sub, Box::new(x), Box::new(y))
            }),
        ))
    })
}
