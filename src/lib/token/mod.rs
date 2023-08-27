use std::fmt;

// // page 12
// const ILLEGAL: &str = "ILLEGAL";
// const EOF: &str = "EOF";
//
// // Identifiers + literals
// const IDENT: &str = "IDENT";
// const INT: &str = "INT";
//
// // Operators
// const ASSIGN: &str = "=";
// const PLUS: &str = "+";
// const COMMA: &str = ",";
// const SEMICOLON: &str = ";";
// const LPAREN: &str = "(";
// const RPAREN: &str = ")";
// const LBRACE: &str = "{";
// const RBRACE: &str = "}";
//
// // keywords
// const FUNCTION: &str = "FUNCTION";
// const LET: &str = "LET";

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    // Identifiers + literals
    IDENT,
    INT,

    // Operators
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    LT,
    GT,
    EQ,
    NOT_EQ,

    // Delimiters
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // keywords
    FUNCTION,
    LET,
    IF,
    TRUE,
    FALSE,
    ELSE,
    RETURN
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::ILLEGAL => write!(f, "ILLEGAL"),
            TokenType::EOF => write!(f, "EOF"),
            TokenType::IDENT => write!(f, "IDENT"),
            TokenType::INT => write!(f, "INT"),
            TokenType::ASSIGN => write!(f, "ASSIGN"),
            TokenType::PLUS => write!(f, "PLUS"),
            TokenType::MINUS => write!(f, "MINUS"),
            TokenType::BANG => write!(f, "BANG"),
            TokenType::ASTERISK => write!(f, "ASTERISK"),
            TokenType::SLASH => write!(f, "SLASH"),
            TokenType::GT => write!(f, "GT"),
            TokenType::LT => write!(f, "LT"),
            TokenType::EQ => write!(f, "EQ"),
            TokenType::NOT_EQ => write!(f, "NOT_EQ"),
            TokenType::COMMA => write!(f, "COMMA"),
            TokenType::SEMICOLON => write!(f, "SEMICOLON"),
            TokenType::LPAREN => write!(f, "LPAREN"),
            TokenType::RPAREN => write!(f, "RPAREN"),
            TokenType::LBRACE => write!(f, "LBRACE"),
            TokenType::RBRACE => write!(f, "RBRACE"),
            TokenType::FUNCTION => write!(f, "FUNCTION"),
            TokenType::LET => write!(f, "LET"),
            TokenType::IF => write!(f, "IF"),
            TokenType::TRUE => write!(f, "TRUE"),
            TokenType::FALSE => write!(f, "FALSE"),
            TokenType::ELSE => write!(f, "ELSE"),
            TokenType::RETURN => write!(f, "RETURN")
            // Add more match cases for other variants as needed
        }
    }
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
   pub fn new(token_type: TokenType, character: char) -> Token {
        Token { token_type, literal: character.to_string() }
    }
}

impl Clone for Token {
    fn clone(&self) -> Self {
        Token { token_type: self.token_type.clone(), literal: self.literal.clone() }
    }
}
