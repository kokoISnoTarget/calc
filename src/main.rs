use std::{
    io::{Read, Write},
    process::ExitCode,
};

use chumsky::{
    Parser,
    input::{Input, Stream},
};
use clap::Parser as _;
use logos::Logos;

use crate::{opts::Options, token::Token};

pub mod eval;
pub mod opts;
pub mod parse;
pub mod token;

fn main() -> ExitCode {
    let options = Options::parse();

    let input_expr;

    if let Some(input) = options.input {
        input_expr = std::fs::read_to_string(input).expect("Failed to read file");
    } else if !options.expr.is_empty() {
        input_expr = options.expr.join("");
    } else {
        let mut tmp = String::new();
        std::io::stdin()
            .read_to_string(&mut tmp)
            .expect("Failed to read stdin");
        input_expr = tmp;
    }

    let token_iter = Token::lexer(&input_expr)
        .spanned()
        .map(|(tok, span)| match tok {
            Ok(tok) => (tok, span.into()),
            Err(()) => (Token::Error, span.into()),
        });

    let token_stream =
        Stream::from_iter(token_iter).map((0..input_expr.len()).into(), |(t, s): (_, _)| (t, s));

    let ast = match parse::parser().parse(token_stream).into_result() {
        Ok(result) => result,
        Err(err) => {
            eprintln!("Parse error: {:?}", err);
            return ExitCode::FAILURE;
        }
    };

    let result = eval::eval(ast);
    let value = match result {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Evaluation error: {:?}", err);
            return ExitCode::FAILURE;
        }
    };

    if let Some(output) = options.output {
        std::fs::write(output, value.to_string()).expect("Failed to write output");
    } else {
        let mut stdout = std::io::stdout().lock();
        writeln!(stdout, "{}", value).expect("Failed to write to stdout");
        let flush_result = stdout.flush();

        if flush_result.is_err() {
            return ExitCode::FAILURE;
        }
    }

    ExitCode::SUCCESS
}
