# logoc

Compiler for the [LOGO educational programming language][logo-lang].

The [Rust programming language][rust] is used as an intermediate representation
for code generation. We take advantage of the [Rust turtle crate][turtle] for
the graphics. All type checking and error handling is done within the logoc
compiler.

## Example

Let's say you have the following logo program that draws a square.

```logo
fd 100
rt 90
bk 100
rt 90
fd 100
lt 90
fd 100
lt 90
```

logoc will compile it into a valid Rust program that looks something like this:

```rust
#![deny(warnings)]
extern crate turtle;
fn main() {
    let mut _turtle = ::turtle::Turtle::new();
    _turtle.forward(100f64);
    _turtle.right(90f64);
    _turtle.forward(100f64);
    _turtle.right(90f64);
    _turtle.forward(100f64);
    _turtle.right(90f64);
    _turtle.forward(100f64);
    _turtle.right(90f64);
}
```

logoc will then automatically run cargo (the Rust build tool) and build the
generated Rust code into an executable.

## Project Status

This is currently in the proof-of-concept phase. The program to be compiled is
currently [hardcoded](https://github.com/sunjay/logoc/blob/master/src/bin/logoc.rs#L48)
until we add support for parsing LOGO programs. You can modify the hardcoded
program however you want to see how the generated program changes.

## Building & Compiling

Make sure you have [Rust](https://rustup.rs/) installed. Run the compiler using
the following command:

```bash
$ cargo run examples/square.logo
```

To format the generated Rust code, make sure you have
[rustfmt](https://github.com/rust-lang-nursery/rustfmt) installed and then add
the `--fmt` option:

```bash
$ cargo run examples/square.logo -- --fmt
```

You can run the generated executable and see the program running using:

```bash
$ ./build/target/debug/square
```

[logo-lang]: https://en.wikipedia.org/wiki/Logo_(programming_language)
[rust]: https://www.rust-lang.org/
[turtle]: http://turtle.rs/
