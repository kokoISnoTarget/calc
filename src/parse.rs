use chumsky::input::ValueInput;
use chumsky::pratt::{infix, left, postfix};
use chumsky::{
    Parser,
    pratt::{prefix, right},
    prelude::{recursive, *},
};
use rug::Float;

use crate::token::{Constant, Function, Identifier, Token};

pub enum Ast {
    Number(Float),
    Constant(Constant),
    BinOp(BinaryOperation, Box<Ast>, Box<Ast>),
    UnaryOp(UnaryOperation, Box<Ast>),
    Function(Function, Box<Ast>),
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
            Token::Identifier(Identifier::Constant(Constant::Pi)) => Ast::Constant(Constant::Pi),
            Token::Identifier(Identifier::Constant(Constant::Inf)) => Ast::Constant(Constant::Inf),
            Token::Identifier(Identifier::Constant(Constant::Nan)) => Ast::Constant(Constant::Nan),
        };

        let paren = expr
            .clone()
            .delimited_by(just(Token::LeftParen), just(Token::RightParen))
            .map(|e| Ast::Paren(Box::new(e)));

        let function_name = select! {
            Token::Identifier(Identifier::Function(name)) => name,
        };

        let function = function_name
            .then(
                expr.clone()
                    .delimited_by(just(Token::LeftParen), just(Token::RightParen)),
            )
            .map(|(name, args)| Ast::Function(name, Box::new(args)));

        let atom = num.or(paren).or(function);

        atom.pratt((
            // Prefix unary operators
            prefix(3, just(Token::Plus), |_, x, _| {
                Ast::UnaryOp(UnaryOperation::Positive, Box::new(x))
            }),
            prefix(3, just(Token::Minus), |_, x, _| {
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
            infix(left(2), just(Token::Star), |x, _, y, _| {
                Ast::BinOp(BinaryOperation::Mul, Box::new(x), Box::new(y))
            }),
            infix(left(2), just(Token::Slash), |x, _, y, _| {
                Ast::BinOp(BinaryOperation::Div, Box::new(x), Box::new(y))
            }),
            infix(left(2), just(Token::Mod), |x, _, y, _| {
                Ast::BinOp(BinaryOperation::Mod, Box::new(x), Box::new(y))
            }),
            infix(left(2), just(Token::Plus), |x, _, y, _| {
                Ast::BinOp(BinaryOperation::Add, Box::new(x), Box::new(y))
            }),
            infix(left(1), just(Token::Minus), |x, _, y, _| {
                Ast::BinOp(BinaryOperation::Sub, Box::new(x), Box::new(y))
            }),
        ))
    })
}
