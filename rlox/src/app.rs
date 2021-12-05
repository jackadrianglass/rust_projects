use crate::lexer::Lexer;
use std::fs;
use std::io::{self, Write};

pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Self { had_error: false }
    }

    fn run(&self, source: &str) {
        for tok in Lexer::new(source) {
            println!("{:?}", tok);
        }
    }

    pub fn error(&mut self, line: i32, msg: &str) {
        self.report(line, "", msg);
    }

    fn report(&mut self, line: i32, location: &str, msg: &str) {
        eprintln!("[line {}] Error {}: {}", line, location, msg);
        self.had_error = true;
    }

    pub fn run_script(&self, path: &str) {
        let contents = fs::read_to_string(path).expect("Cannot find script path");
        self.run(&contents);
    }

    pub fn run_repl(&mut self) {
        loop {
            let mut line = String::new();
            print!(">");
            io::stdout().flush().unwrap();
            if let Ok(bytes) = io::stdin().read_line(&mut line) {
                if bytes == 0 {
                    println!("Bye!");
                    break;
                }
                self.run(&line.trim());
                self.had_error = false;
            } else {
                println!("Something went wrong!");
                break;
            }
        }
    }
}
