#[cfg(test)]
use crate::lexer::Lexer;
use crate::ast;
use crate::ast::Node;
use crate::parser::Parser;

fn test_let_statement(statement: &ast::Statement, name: &str) -> bool {
    match statement {
        ast::Statement::LetStatement(let_stmt) => {
            if let_stmt.token_literal() != "let" {
                eprintln!("s.token_literal not 'let'. got={}", let_stmt.token_literal());
                return false;
            } else if let_stmt.name.value != name {
                eprintln!("let_stmt.name.value not '{}'. got={}", name, let_stmt.name.value);
                return false;
            } else if let_stmt.name.token_literal() != name {
                eprintln!("let_stmt.name.token_literal() not '{}'. got={}", name, let_stmt.name.token_literal());
                return false;
            } else {
                true
            }
        }
        _ => false,
    }
}

#[test]
fn test_let_statements() {
    let input = "let x = 5;\
let y = 10;\
let foobar = 838383;";

    let lex = Lexer::new(input.to_string());
    let mut parser = Parser::new(lex);
    let program = parser.parse_program().unwrap_or_else(|| panic!("parse_program() returned None"));
    check_parser_errors(parser);

    if program.statements.len() != 3 {
        panic!("program.statements does not contain 3 statements. got={}", program.statements.len());
    }

    let tests = vec!["x", "y", "foobar"];

    for (i, tt) in tests.iter().enumerate() {
        let stmt = program.statements.get(i).unwrap();
        if !test_let_statement(stmt, tt) {
            return;
        }
    }
}

fn check_parser_errors(parser: Parser) {
    let errors = parser.errors();
    if errors.len() == 0 {
        return;
    }

    println!("parser has {} errors", errors.len());
    for msg in errors {
        println!("parser error: {}", msg);
    }
    panic!();
}

#[test]
fn test_return_statements() {
    let input = "return 5;\
return 10;\
return 993322;";

    let lex = Lexer::new(input.to_string());
    let mut parser = Parser::new(lex);

    let program = parser.parse_program().unwrap_or_else(|| panic!("parse_program() returned None"));

    if program.statements.len() != 3 {
        panic!("program.statements does not contain 3 statements. got={}", program.statements.len());
    }

    for (_, statement) in program.statements.iter().enumerate() {
        match statement {
            ast::Statement::ReturnStatement(return_stmt) => {
                if return_stmt.token_literal() != "return" {
                    eprintln!("return_stmt.token_literal not 'return', got {}", return_stmt.token_literal());
                    return;
                }
            }
            _ => {
                eprintln!("statement not ast::Statement::ReturnStatement, got {:?}", statement);
                return;
            }
        }
    }
}

#[test]
fn test_identifier_expression() {
    let input = "foobar;";

    let lex = Lexer::new(input.to_string());
    let mut parser = Parser::new(lex);
    let program = parser.parse_program().unwrap_or_else(|| panic!("parse_program() returned None"));
    check_parser_errors(parser);

    if program.statements.len() != 1 {
        panic!("program has not enough statements. got={}", program.statements.len());
    }

    match &program.statements[0] {
        ast::Statement::ExpressionStatement(stmt) => {
            match &stmt.expression {
                Some(ast::Expression::Identifier(ident)) => {
                    if ident.value != "foobar" {
                        eprintln!("ident.Value not {}. got={}", "foobar", ident.value);
                    }
                    if ident.token_literal() != "foobar" {
                        eprintln!("ident.TokenLiteral not {}. got={}", "foobar", ident.token_literal());
                    }
                }
                _ => panic!("exp not *ast.Identifier. got={:?}", stmt.expression),
            }
        }
        _ => panic!("program.Statements[0] is not ast.ExpressionStatement. got={:?}", program.statements[0]),
    }
}

#[test]
fn test_integer_literal_expression() {
    let input = String::from("5");

    let lex = Lexer::new(input);
    let mut p = Parser::new(lex);
    let program = p.parse_program().unwrap_or_else(|| panic!("parse_program() returned None"));
    check_parser_errors(p);

    if program.statements.len() != 1 {
        panic!("program has not enough statements. got={}", program.statements.len())
    } 

    match &program.statements[0] {
        ast::Statement::ExpressionStatement(stmt) => {
            match &stmt.expression {
                Some(ast::Expression::IntegerLiteral(int)) => {
                    if int.value != 5 {
                        eprintln!("int.Value not {}. got={}", 5, int.value);
                    }
                    if int.token_literal() != "5" {
                        eprintln!("int.TokenLiteral not {}. got={}", "5", int.token_literal());
                    }

                }
            _ => panic!("exp not ast::IntegerLiteral. got={:?}", stmt.expression)
            }
        }
        _ => panic!("program.Statements.[0] is not ast::ExpressionStatement. got={:?}", program.statements[0]),

    }
}