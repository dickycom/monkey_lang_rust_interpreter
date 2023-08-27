#[cfg(test)]
mod mod_test;

use crate::token::{Token, TokenType};

fn is_letter(character: char) -> bool {
    matches!(character, 'a'..='z' | 'A'..='Z' | '_')
}

fn look_up_identifier(identifier: &str) -> TokenType {
    match identifier {
        "let" => TokenType::LET,
        "fn" => TokenType::FUNCTION,
        "if" => TokenType::IF,
        "true" => TokenType::TRUE,
        "false" => TokenType::FALSE,
        "else" => TokenType::ELSE,
        "return" => TokenType::RETURN,
        _ => TokenType::IDENT,
    }
}

fn is_digit(ch: char) -> bool {
    ch.is_ascii_digit()
}


pub struct Lexer {
    input: String,
    position: usize,
    // current position in input (points to current char)
    read_position: usize,
    // current reading position in input (after current char)
    ch: char, // current char under examination
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer { input, position: 0, read_position: 0, ch: '\0' };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            '=' => {
                if self.peak_char() != '=' {
                    Token { token_type: TokenType::ASSIGN, literal: self.ch.to_string() }
                } else {
                    let ch = self.ch;
                    self.read_char();
                    let literal = ch.to_string() + &*self.ch.to_string();
                    Token { token_type: TokenType::EQ, literal }
                }
            },
            '+' => Token::new(TokenType::PLUS, self.ch),
            '-' => Token::new(TokenType::MINUS, self.ch),
            '!' => {
                if self.peak_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    let literal = ch.to_string() + &*self.ch.to_string();
                    Token { token_type: TokenType::NOT_EQ, literal }
                } else {
                    Token { token_type: TokenType::BANG, literal: self.ch.to_string() }
                }
            },
            '/' => Token::new(TokenType::SLASH, self.ch),
            '*' => Token::new(TokenType::ASTERISK, self.ch),
            '<' => Token::new(TokenType::LT, self.ch),
            '>' => Token::new(TokenType::GT, self.ch),
            ';' => Token::new(TokenType::SEMICOLON, self.ch),
            '(' => Token::new(TokenType::LPAREN, self.ch),
            ')' => Token::new(TokenType::RPAREN, self.ch),
            ',' => Token::new(TokenType::COMMA, self.ch),
            '{' => Token::new(TokenType::LBRACE, self.ch),
            '}' => Token::new(TokenType::RBRACE, self.ch),
            '\0' => Token::new(TokenType::EOF, self.ch),
            _ => {
                return if is_letter(self.ch) {
                    let identifier = self.read_identifier();
                    Token { token_type: look_up_identifier(identifier), literal: identifier.to_string() }
                } else if is_digit(self.ch) {
                    Token { token_type: TokenType::INT, literal: self.read_number().to_string() }
                } else {
                    Token::new(TokenType::ILLEGAL, self.ch)
                };
            }
        };
        self.read_char();
        token
    }

    fn read_identifier(&mut self) -> &str {
        let position = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }
        self.input.get(position..self.position).unwrap()
    }

    fn read_number(&mut self) -> &str {
        let position = self.position;
        while is_digit(self.ch) {
            self.read_char();
        }
        self.input.get(position..self.position).unwrap()
    }

    fn skip_whitespace(&mut self) {
        while self.ch == '\r' || self.ch == '\t' || self.ch == ' ' || self.ch == '\n' {
            self.read_char()
        }
    }

    fn peak_char(&mut self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input.chars().nth(self.read_position).unwrap()
        }
    }
}

