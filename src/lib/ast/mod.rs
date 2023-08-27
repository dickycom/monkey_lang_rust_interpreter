mod mod_test;

use crate::token;
use crate::token::Token;

pub trait Node {
    fn token_literal(&self) -> String;
    fn to_string(&self) -> String;
}

pub trait StatementNode: Node {
    fn statement_node(&self);
}

pub trait ExpressionNode: Node {
    fn expression_node(&self);
}

#[derive(Debug)]
pub struct Expression {}

impl Node for Expression {
    fn token_literal(&self) -> String {
        return String::new();
    }
    fn to_string(&self) -> String {
        return String::new();
    }
}

#[derive(Debug)]
pub enum Statement {
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement),
    ExpressionStatement(ExpressionStatement),
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::LetStatement(stmt) => stmt.token_literal(),
            Statement::ReturnStatement(stmt) => stmt.token_literal(),
            Statement::ExpressionStatement(stmt) => stmt.token_literal(),
            // Handle other variants similarly
        }
    }

    fn to_string(&self) -> String {
        match self {
            Statement::LetStatement(stmt) => stmt.to_string(),
            Statement::ReturnStatement(stmt) => stmt.to_string(),
            Statement::ExpressionStatement(stmt) => stmt.to_string(),
            // Handle other variants similarly
        }
    }
}

pub struct Program {
    pub statements: Vec<Statement>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        match self.statements.first().unwrap() {
            Statement::LetStatement(stmt) => stmt.token_literal(),
            Statement::ReturnStatement(stmt) => stmt.token_literal(),
            Statement::ExpressionStatement(stmt) => stmt.token_literal(),
            // Handle other variants similarly
        }
    }

    fn to_string(&self) -> String {
        let mut out = String::new();
        for stmt in &self.statements {
            out.push_str(&stmt.to_string());
        }
        out
    }

}

#[derive(Debug)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Expression,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        return self.token.literal.clone();
    }
    fn to_string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.token_literal());
        out.push_str(" ");
        out.push_str(&self.name.to_string());
        out.push_str(" = ");
        out.push_str(&self.value.to_string());
        out.push_str(";");
        out
    }
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub token: token::Token,
    pub return_value: Expression,
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        return self.token.literal.clone();
    }
    fn to_string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.token_literal());
        out.push_str(" ");
        out.push_str(&self.return_value.to_string());
        out.push_str(";");
        out
    }
}

#[derive(Debug)]
pub struct ExpressionStatement {
    token: Token,
    expression: Expression
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        return self.token.literal.clone();
    }
    fn to_string(&self) -> String {
        return self.expression.to_string();
    }
}


#[derive(Debug)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        return self.token.literal.clone();
    }
    fn to_string(&self) -> String {
        return self.value.clone();
    }
}

