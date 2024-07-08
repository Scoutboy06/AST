use std::borrow::Borrow;

use crate::{
    ast::{ASTNode, BinaryExpression, Expression},
    lexer::{Lexer, Token, TokenType},
};

pub struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    current_token: Box<Token>,
}

#[derive(Debug)]
pub enum ParserError {
    InvalidCharacter(Box<Token>),
    EmptyFile(Box<Token>),
    InternalError(Box<Token>),
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Self {
        let current_token = lexer.next_token();

        Self {
            lexer,
            current_token: Box::new(current_token),
        }
    }

    fn advance(&mut self) {
        self.current_token = Box::new(self.lexer.next_token());
    }

    pub fn parse(&mut self) -> Result<Box<ASTNode>, ParserError> {
        match self.current_token.token_type {
            TokenType::Number | TokenType::Paren => {
                let expression = self.parse_expression()?;
                Ok(Box::new(ASTNode::BinaryExpression(expression)))
            }
            TokenType::Operator => Err(ParserError::InvalidCharacter(self.current_token.clone())),
            TokenType::EndOfFile => Err(ParserError::EmptyFile(self.current_token.clone())),
            TokenType::Invalid => Err(ParserError::InvalidCharacter(self.current_token.clone())),
        }
    }

    /* This method will handle the most basic component of an arithmetic
    expression, such as number, (variables,) and parenthesized expressions */
    fn parse_factor(&mut self) -> Result<BinaryExpression, ParserError> {
        let left = self.current_token.clone();
        self.advance();

        match left.token_type {
            TokenType::Number => {
                if left.value.contains('.') {
                    return match left.value.parse::<f64>() {
                        Ok(num) => Ok(Expression::Float(num)),
                        Err(_) => Err(ParserError::InvalidCharacter(left)),
                    };
                }

                return match left.value.parse::<i64>() {
                    Ok(num) => Ok(Expression::Int(num)),
                    Err(_) => Err(ParserError::InvalidCharacter(left)),
                };
            }
            TokenType::Paren => match left.value.as_str() {
                "(" => self.parse_expression(),
                _ => Err(ParserError::InvalidCharacter(left)),
            },
            _ => Err(ParserError::InvalidCharacter(left)),
        }
    }

    /* This method will handle multiplication and division, which have
    higher precedence than addition and subtraction */
    fn parse_term(&mut self) -> Result<BinaryExpression, ParserError> {
        let mut left = self.parse_factor()?;

        while self.current_token.token_type == TokenType::Operator
            && (self.current_token.value.as_str() == "*"
                || self.current_token.value.as_str() == "/")
        {
            let operator = self.current_token.value.clone();
            self.advance();
            let right = self.parse_factor()?;
            left = Expression::BinaryOperation(Box::new(BinaryOperation {
                left: Box::new(left),
                right: Box::new(right),
                operator,
            }));
        }

        Ok(left)
    }

    /* This method wil handle addition and subtraction, which have the lowest precedence */
    fn parse_expression(&mut self) -> Result<BinaryExpression, ParserError> {
        let mut term = self.parse_term()?;

        while self.current_token.token_type == TokenType::Operator
            && (self.current_token.value.as_str() == "+"
                || self.current_token.value.as_str() == "-")
        {
            let operator = self.current_token.value.clone();
            self.advance();
            let next_term = self.parse_term()?;
            term = Expression::BinaryOperation(Box::new(BinaryOperation {
                left: Box::new(term),
                right: Box::new(next_term),
                operator,
            }));
        }

        Ok(term)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{ASTNode, BinaryOperationResult, Expression},
        lexer::Lexer,
    };

    use super::Parser;

    #[test]
    fn addition() {
        let mut lexer = Lexer::new("3 + 4 + 5");
        let mut parser = Parser::new(&mut lexer);
        let tree = parser.parse();

        if let Err(err) = &tree {
            println!("Error from parsing: {:?}", err);
            panic!();
        }

        let result = match tree.unwrap().as_ref() {
            t => match t {
                ASTNode::Expression(exp) => match exp {
                    Expression::BinaryOperation(operation) => match operation.evaluate() {
                        BinaryOperationResult::Int(num) => num,
                        _ => panic!("Wrong result type"),
                    },
                    _ => panic!("Not a binary operation"),
                },
                _ => panic!("Not an expression"),
            },
        };

        assert_eq!(result, 12);
    }

    #[test]
    fn subtraction() {
        let mut lexer = Lexer::new("23 - 12 - 2");
        let mut parser = Parser::new(&mut lexer);
        let tree = parser.parse();

        if let Err(err) = &tree {
            println!("Error from parsing: {:?}", err);
            panic!();
        }

        let result = match tree.unwrap().as_ref() {
            t => match t {
                ASTNode::Expression(exp) => match exp {
                    Expression::BinaryOperation(operation) => match operation.evaluate() {
                        BinaryOperationResult::Int(num) => num,
                        _ => panic!("Wrong result type"),
                    },
                    _ => panic!("Not a binary operation"),
                },
                _ => panic!("Not an expression"),
            },
        };

        assert_eq!(result, 9);
    }

    #[test]
    fn multiplication() {
        let mut lexer = Lexer::new("3 * 4 * 5");
        let mut parser = Parser::new(&mut lexer);
        let tree = parser.parse();

        if let Err(err) = &tree {
            println!("Error from parsing: {:?}", err);
            panic!();
        }

        let result = match tree.unwrap().as_ref() {
            t => match t {
                ASTNode::Expression(exp) => match exp {
                    Expression::BinaryOperation(operation) => match operation.evaluate() {
                        BinaryOperationResult::Int(num) => num,
                        _ => panic!("Wrong result type"),
                    },
                    _ => panic!("Not a binary operation"),
                },
                _ => panic!("Not an expression"),
            },
        };

        assert_eq!(result, 60);
    }

    #[test]
    fn division() {
        let mut lexer = Lexer::new("20 / 5 / 2");
        let mut parser = Parser::new(&mut lexer);
        let tree = parser.parse();

        if let Err(err) = &tree {
            println!("Error from parsing: {:?}", err);
            panic!();
        }

        let result = match tree.unwrap().as_ref() {
            t => match t {
                ASTNode::Expression(exp) => match exp {
                    Expression::BinaryOperation(operation) => match operation.evaluate() {
                        BinaryOperationResult::Int(num) => num,
                        _ => panic!("Not an integer type"),
                    },
                    _ => panic!("Not a binary operation"),
                },
                _ => panic!("Not an expression"),
            },
        };

        assert_eq!(result, 2);
    }

    #[test]
    fn order_of_operations() {
        let mut lexer = Lexer::new("3 + 4 * 5 - 6");
        let mut parser = Parser::new(&mut lexer);
        let tree = parser.parse();

        if let Err(err) = &tree {
            println!("Error from parsing: {:?}", err);
            panic!();
        }

        let result = match tree.unwrap().as_ref() {
            t => match t {
                ASTNode::Expression(exp) => match exp {
                    Expression::BinaryOperation(operation) => match operation.evaluate() {
                        BinaryOperationResult::Int(num) => num,
                        _ => panic!("Wrong result type"),
                    },
                    _ => panic!("Not a binary operation"),
                },
                _ => panic!("Not an expression"),
            },
        };

        assert_eq!(result, 17);
    }
}
