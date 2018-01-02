use quote::{Tokens, Ident};

use ast::{Program, Instruction, Expr};

#[derive(Debug, Clone)]
pub enum Error {
}

pub fn generate_rust(program: Program) -> Result<Tokens, Error> {
    let stmts = program.into_iter()
        .map(generate_instr)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(quote! {
        extern crate turtle;

        fn main() {
            let mut turtle = ::turtle::Turtle::new();
            #(#stmts)*
        }
    })
}

fn generate_instr(instr: Instruction) -> Result<Tokens, Error> {
    use self::Instruction::*;
    let (method, value): (Ident, _) = match instr {
        Forward(Expr::Number(value)) => ("forward".into(), value),
        Backward(Expr::Number(value)) => ("backward".into(), value),
        Left(Expr::Number(value)) => ("left".into(), value),
        Right(Expr::Number(value)) => ("right".into(), value),
    };
    Ok(quote! {
        turtle.#method(#value);
    })
}
