use quote::{Tokens, Ident};

use ast::{Program, Instruction, Expr};

#[derive(Debug, Clone)]
pub enum Error {
}

pub fn to_tokens(program: Program) -> Result<Tokens, Error> {
    let stmts = program.into_iter()
        .map(instr)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(quote! {
        #![deny(warnings)]

        extern crate turtle;

        fn main() {
            let mut _turtle = ::turtle::Turtle::new();
            #(#stmts)*
        }
    })
}

fn instr(instr: Instruction) -> Result<Tokens, Error> {
    use self::Instruction::*;
    match instr {
        Forward(arg) => call_method("forward", arg),
        Backward(arg) => call_method("backward", arg),
        Left(arg) => call_method("left", arg),
        Right(arg) => call_method("right", arg),
    }
}

fn call_method(method: &str, expr: Expr) -> Result<Tokens, Error> {
    let method = Ident::from(method);
    let value = match expr {
        Expr::Number(value) => quote! { #value },
    };

    Ok(quote! {
        _turtle.#method(#value);
    })
}
