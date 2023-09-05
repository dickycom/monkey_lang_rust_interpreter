use crate::ast::{Expression, Identifier, LetStatement, Node, Program, ReturnStatement, Statement};
use crate::token::{Token, TokenType};


#[test]
fn test_to_string() {
    let program = Program {
        statements: vec![
            Statement::LetStatement(LetStatement {
                token: Token {
                    token_type: TokenType::LET,
                    literal: String::from("let"),
                },
                name: Identifier {
                    token: Token {
                        token_type: TokenType::IDENT,
                        literal: String::from("myVar"),
                    },
                    value: String::from("myVar"),
                },
                value: None,
            }),
            Statement::ReturnStatement(ReturnStatement {
                token: Token {
                    token_type: TokenType::RETURN,
                    literal: String::from("return"),
                },
                return_value: None,
            }),
        ],
    };
    if program.to_string() != "let myVar = ;return ;" {
        panic!("program.to_string() wrong. got={}", program.to_string());
    }
}