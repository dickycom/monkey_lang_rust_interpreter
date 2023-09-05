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
pub enum Expression {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    PrefixExpression(PrefixExpression),
    InfixExpression(InfixExpression),
    Boolean(Boolean),
    // Add other variants similarly
}

#[derive(Debug)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> String {
        return self.token.literal.clone();
    }
    fn to_string(&self) -> String {
        return self.value.to_string();
    }
}

#[derive(Debug)]
pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Box<Expression>,
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> String {
        return self.token.literal.clone();
    }
    fn to_string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.operator);
        out.push_str(&self.right.to_string());
        out
    }
}

#[derive(Debug)]
pub struct InfixExpression {
    pub token: Token,
    pub left: Box<Expression>,
    pub operator: String,
    pub right: Box<Expression>,
}

impl Node for InfixExpression {
    fn token_literal(&self) -> String {
        return self.token.literal.clone();
    }
    fn to_string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.left.to_string());
        out.push_str(&self.operator);
        out.push_str(&self.right.to_string());
        out
    }
}

#[derive(Debug)]
pub struct Boolean {
    pub token: Token,
    pub value: bool,
}

impl Node for Boolean {
    fn token_literal(&self) -> String {
        return self.token.literal.clone();
    }
    fn to_string(&self) -> String {
        return self.value.to_string();
    }
}

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
    pub value: Option<Expression>,
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

        if let Some(value) = &self.value {
            out.push_str(&value.to_string());
        } else {
            out.push_str("");
        }

        out.push_str(";");
        out
    }
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub token: token::Token,
    pub return_value: Option<Expression>,
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        return self.token.literal.clone();
    }
    fn to_string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.token_literal());
        out.push_str(" ");
        if let Some(return_value) = &self.return_value {
            out.push_str(&return_value.to_string());
        } else {
            out.push_str("");
        }

        out.push_str(";");
        out
    }
}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Option<Expression>,
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        return self.token.literal.clone();
    }
    fn to_string(&self) -> String {
        match &self.expression {
            Some(expression) => expression.to_string(),
            None => String::from("expression not_found"),
        }
    }
}

#[derive(Debug)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn to_string(&self) -> String {
        self.value.clone()
    }
}
