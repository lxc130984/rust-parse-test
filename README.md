# rust-parse-test

A minimal interpreted language written in Rust for study, built with [chumsky](https://crates.io/crates/chumsky) for parsing and [ariadne](https://crates.io/crates/ariadne) for error reporting.

## Features

- **REPL** — interactive line-by-line evaluation with history (`>> ` prompt).
- **File evaluation** — pass a source file as the first argument to run it directly.
- **Arithmetic** — `+`, `-`, `*`, `/`, unary negation.
- **Comparisons** — `==`, `!=`, `>`, `<`, `>=`, `<=`.
- **Logic** — `&&`, `||`.
- **Literals** — integer/float numbers, `true` / `false` booleans.
- **Let bindings** — `let x = 1; x + 2`
- **Blocks** — `{ expr1; expr2; expr3 }` returns the last value.
- **Conditionals** — `if cond { ... } else { ... }`
- **Functions** — `fn name(arg1, arg2) { body } rest` with recursion support.

## Quick Start

```bash
cargo run              # start the REPL
cargo run -- file.txt  # evaluate a source file
```

## Example

```
>> let x = 10; x * 2 + 1
21
>> if true { 42 } else { 0 }
42
>> fn factorial(n) { if n <= 1 { 1 } else { n * factorial(n - 1) } } factorial(5)
120
>> 1 + true
eval error: error_kind : Add
```

## Project Structure

| File          | Purpose                                                                      |
|---------------|------------------------------------------------------------------------------|
| `src/main.rs` | Entry point, REPL, file loader                                               |
| `src/lib.rs`  | Module re-exports                                                            |
| `src/ast.rs`  | Parser (chumsky combinators) and parse error reporting                       |
| `src/eval.rs` | AST types (`Expr`, `Value`), evaluator, environment, runtime error reporting |
| `src/test`    | Example factorial program                                                    |

## Dependencies

- **chumsky** `0.12` — parser combinators
- **ariadne** `0.6` — pretty error diagnostics
- **rustyline** `18` — line editor with history for the REPL


