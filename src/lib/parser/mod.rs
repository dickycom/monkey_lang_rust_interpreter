mod mod_test;

use std::collections::HashMap;
use crate::{ast, lexer, token};

type PrefixParseFn = fn(&mut Parser) -> Option<ast::Expression>;
type InfixParseFn = fn(&mut Parser, ast::Expression) -> Option<ast::Expression>;

pub struct Parser {
    lexer: lexer::Lexer,
    current_token: token::Token,
    peek_token: token::Token,
    errors: Vec<String>,
    prefix_parse_fns: HashMap<token::TokenType, PrefixParseFn>,
    infix_parse_fns: HashMap<token::TokenType, InfixParseFn>,
}

impl Parser {
    pub fn new(mut lexer: lexer::Lexer) -> Parser {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();
        Parser { lexer, current_token, peek_token, errors: Vec::new(), prefix_parse_fns: HashMap::new(), infix_parse_fns: HashMap::new() }
    }

    pub fn next_token(&mut self) {
        self.current_token = std::mem::replace(&mut self.peek_token, self.lexer.next_token());
    }

    pub fn parse_statement(&mut self) -> Option<ast::Statement> {
        match self.current_token.token_type {
            token::TokenType::LET => self.parse_let_statement(),
            token::TokenType::RETURN => self.parse_return_statement(),
            _ => None
        }
    }

    pub fn errors(&self) -> Vec<String> {
        self.errors.clone()
    }

    pub fn peek_error(&mut self, token_type: token::TokenType) {
        let error = format!("expected next token to be {:?}, got {:?} instead", token_type, self.peek_token.token_type);
        self.errors.push(error);
    }

    pub fn parse_let_statement(&mut self) -> Option<ast::Statement> {
        let current_token = self.current_token.clone();

        if !self.expect_peek(token::TokenType::IDENT) {
            return None;
        }

        let name = ast::Identifier { token: self.current_token.clone(), value: self.current_token.literal.clone() };

        if !self.expect_peek(token::TokenType::ASSIGN) {
            return None;
        }

        while !self.current_token_is(token::TokenType::SEMICOLON) {
            self.next_token();
        };

        let expression = ast::Expression {};
        let statement = ast::LetStatement { token: current_token.clone(), name, value: expression };

        Some(ast::Statement::LetStatement(statement))
    }

    pub fn parse_return_statement(&mut self) -> Option<ast::Statement> {
        let current_token = self.current_token.clone();
        self.next_token();

        while !self.current_token_is(token::TokenType::SEMICOLON) {
            self.next_token();
        };

        let statement = ast::ReturnStatement { token: current_token.clone(), return_value: ast::Expression {} };
        return Some(ast::Statement::ReturnStatement(statement));
    }

    pub fn current_token_is(&self, token_type: token::TokenType) -> bool {
        self.current_token.token_type == token_type
    }

    pub fn peek_token_is(&self, token_type: token::TokenType) -> bool {
        self.peek_token.token_type == token_type
    }

    pub fn expect_peek(&mut self, token_type: token::TokenType) -> bool {
        if self.peek_token_is(token_type) {
            self.next_token();
            true
        } else {
            self.peek_error(token_type);
            false
        }
    }


    pub fn parse_program(&mut self) -> Option<ast::Program> {
        let mut program = ast::Program { statements: vec![] };
        while !self.current_token_is(token::TokenType::EOF) {
            let statement = self.parse_statement();
            if let Some(..) = statement {
                program.statements.push(statement.unwrap());
            }
            self.next_token();
        }
        Some(program)
    }

    pub fn register_prefix(&mut self, token_type: token::TokenType, func: PrefixParseFn) {
        self.prefix_parse_fns.insert(token_type, func);
    }

    pub fn register_infix(&mut self, token_type: token::TokenType, func: InfixParseFn) {
        self.infix_parse_fns.insert(token_type, func);
    }
}
