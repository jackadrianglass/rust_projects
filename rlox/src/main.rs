use std::env;

pub mod app;
pub mod lexer;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut lox = app::Lox::new();
    if args.len() > 2 {
        println!("rlox <optional path to file>");
    } else if args.len() == 2 {
        lox.run_script(&args[1]);
    } else {
        lox.run_repl();
    }
}
