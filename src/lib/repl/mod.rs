use crate::token;
use crate::lexer;
use std::{ io};
use std::io::{BufRead};


const PROMPT: &str = ">> ";

pub fn start<R: io::BufRead, W: io::Write>(input: R, mut output: W) {
    let scanner = io::BufReader::new(input);
    for line in scanner.lines() {
        write!(output, "{}", PROMPT).expect("Error writing to output");
        let line = line.expect("Error reading line");

        let mut lexer = lexer::Lexer::new(line.to_string());

        loop {
            let tok = lexer.next_token();
            if tok.token_type == token::TokenType::EOF {
                break;
            }
            writeln!(output, "{:?}", tok).expect("Error writing to output");
        }
    }
}

