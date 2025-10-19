use rug::{
    Float,
    ops::{CompleteRound, Pow},
};

use crate::{
    parse::{Ast, BinaryOperation, UnaryOperation},
    token::{Constant, Function},
};

pub fn eval(ast: Ast) -> Result<Float, ()> {
    match ast {
        Ast::Number(float) => Ok(float),
        Ast::BinOp(BinaryOperation::Add, ast, ast1) => Ok(eval(*ast)? + eval(*ast1)?),
        Ast::BinOp(BinaryOperation::Sub, ast, ast1) => Ok(eval(*ast)? - eval(*ast1)?),
        Ast::BinOp(BinaryOperation::Mul, ast, ast1) => Ok(eval(*ast)? * eval(*ast1)?),
        Ast::BinOp(BinaryOperation::Div, ast, ast1) => Ok(eval(*ast)? / eval(*ast1)?),
        Ast::BinOp(BinaryOperation::Pow, ast, ast1) => Ok(eval(*ast)?.pow(eval(*ast1)?)),
        Ast::BinOp(BinaryOperation::Mod, ast, ast1) => Ok(eval(*ast)?.remainder(&eval(*ast1)?)),
        Ast::UnaryOp(UnaryOperation::Positive, ast) => Ok(eval(*ast)?),
        Ast::UnaryOp(UnaryOperation::Negative, ast) => Ok(-eval(*ast)?),
        Ast::UnaryOp(UnaryOperation::Fact, ast) => {
            let out = eval(*ast)?.to_u32_saturating();
            let Some(out) = out else {
                return Err(());
            };
            Ok(Float::factorial(out).complete(64))
        }
        Ast::Paren(ast) => eval(*ast),
        Ast::Constant(constant) => match constant {
            Constant::Pi => Ok(Float::with_val(64, rug::float::Constant::Pi)),
            Constant::Inf => Ok(Float::with_val(64, rug::float::Special::Infinity)),
            Constant::Nan => Ok(Float::with_val(64, rug::float::Special::Nan)),
            _ => Err(()),
        },
        Ast::Function(function, ast) => {
            let ast = eval(*ast)?;

            match function {
                Function::Sin => Ok(ast.sin()),
                Function::Cos => Ok(ast.cos()),
                Function::Tan => Ok(ast.tan()),
                Function::Exp => Ok(ast.exp()),
                Function::Sqrt => Ok(ast.sqrt()),
            }
        }
    }
}
