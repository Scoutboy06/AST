use super::{
    expressions::{BinaryExpression, BinaryOperation, Expression},
    nodes::program::Program,
    statements::VariableDeclaration,
    FunctionDeclaration, Identifier, Literal, Node, NumberLiteral, Shebang, Statement,
    VariableDeclarator, VariableKind,
};
use crate::lexer::{keywords::Keyword, Kind, Lexer, Span, Token, TokenValue};
use string_cache::DefaultAtom as Atom;

pub struct Parser<'a> {
    source: &'a str,
    lexer: Lexer<'a>,
    current_token: Token,
    prev_token_end: usize,
}

#[derive(Debug, PartialEq)]
pub enum ParserError {
    InvalidToken(Token),
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            lexer: Lexer::new(source),
            current_token: Token::default(),
            prev_token_end: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Program, ParserError> {
        let mut body: Vec<Statement> = Vec::new();
        let source_len = self.source.len();

        self.advance(); // Initialize first token

        loop {
            if self.current_token.kind == Kind::Eof {
                break;
            }

            let statement: Statement = self.parse_statement()?;
            body.push(statement);
        }

        Ok(Program {
            node: Node::new(0, source_len),
            shebang: None,
            body,
        })
    }

    fn advance(&mut self) {
        self.prev_token_end = self.current_token.end;
        self.current_token = self.lexer.next_token();
    }

    fn expect_token_kind(&self, kind: Kind, error: ParserError) -> Result<(), ParserError> {
        if self.current_token.kind != kind {
            Err(error)
        } else {
            Ok(())
        }
    }

    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        match self.current_token.kind {
            Kind::Keyword => match self.current_token.value.expect_keyword() {
                Keyword::Var | Keyword::Let | Keyword::Const => {
                    let var_dec = self.parse_variable_declaration()?;
                    Ok(Statement::VariableDeclaration(var_dec.into()))
                }
                Keyword::Function => {
                    let fn_dec = self.parse_function_declaration()?;
                    Ok(Statement::FunctionDeclaration(fn_dec.into()))
                }
                _ => unreachable!(),
            },
            _ => Err(ParserError::InvalidToken(self.current_token.clone())),
        }
    }

    fn parse_variable_declaration(&mut self) -> Result<VariableDeclaration, ParserError> {
        let start_pos = self.current_token.start;

        let kind = match self.current_token.value.expect_keyword() {
            Keyword::Var => VariableKind::Var,
            Keyword::Let => VariableKind::Let,
            Keyword::Const => VariableKind::Const,
            _ => unreachable!(),
        };

        let mut declarations = Vec::new();

        self.advance();

        loop {
            let start = self.current_token.start;

            self.expect_token_kind(
                Kind::Identifier,
                ParserError::InvalidToken(self.current_token.clone()),
            );
            let identifier = Identifier {
                node: Node::new(start, self.current_token.end),
                name: self.current_token.value.expect_string().clone(), // TODO: check if .clone() is cheap?
            };

            self.advance();

            if self.current_token.kind == Kind::Comma {
                declarations.push(VariableDeclarator {
                    node: Node::new(start, self.current_token.end),
                    id: identifier,
                    init: None,
                });

                continue;
            }

            if self.current_token.kind != Kind::Equals {
                break;
            }

            // TODO: support explicit types ("let a: number")

            self.advance();

            let decl = match self.current_token.kind {
                Kind::Number => {
                    let value = self.current_token.value.expect_number();
                    // TODO: support binary operations

                    VariableDeclarator {
                        node: Node::new(start, self.current_token.end),
                        id: identifier,
                        init: Some(NumberLiteral::as_expression(
                            Node::new(self.current_token.start, self.current_token.end),
                            value,
                        )),
                    }
                }
                Kind::String => todo!(),
                Kind::Null => todo!(),
                Kind::OpenBracket => todo!(),
                Kind::OpenBrace => todo!(),
                Kind::OpenParen => todo!(),
                _ => return Err(ParserError::InvalidToken(self.current_token.clone())),
            };

            declarations.push(decl);

            self.advance();

            if self.current_token.kind != Kind::Comma {
                break;
            }
        }

        Ok(VariableDeclaration {
            node: Node::new(start_pos, self.prev_token_end),
            kind,
            declarations,
        })
    }

    fn parse_function_declaration(&mut self) -> Result<FunctionDeclaration, ParserError> {
        todo!()
    }

    /* This method will handle the most basic component of an arithmetic
    expression, such as number, (variables,) and parenthesized expressions */
    // fn parse_factor(&mut self) -> Result<Expression, ParserError> {
    //     let left = self.current_token.clone();
    //     self.advance();

    //     match left.value {
    //         TokenType::Number => {
    //             if left.value.contains('.') {
    //                 return match left.value.parse::<f64>() {
    //                     Ok(num) => Ok(Expression::Float(num)),
    //                     Err(_) => Err(ParserError::InvalidCharacter(left)),
    //                 };
    //             }

    //             return match left.value.parse::<i64>() {
    //                 Ok(num) => Ok(Expression::Int(num)),
    //                 Err(_) => Err(ParserError::InvalidCharacter(left)),
    //             };
    //         }
    //         TokenType::Paren => match left.value.as_str() {
    //             "(" => self.parse_expression(),
    //             _ => Err(ParserError::InvalidCharacter(left)),
    //         },
    //         _ => Err(ParserError::InvalidCharacter(left)),
    //     }
    // }

    /* This method will handle multiplication and division, which have
    higher precedence than addition and subtraction */
    // fn parse_term(&mut self) -> Result<Expression, ParserError> {
    //     let mut left = self.parse_factor()?;

    //     while self.current_token.value == TokenType::Operator
    //         && (self.current_token.value.as_str() == "*"
    //             || self.current_token.value.as_str() == "/")
    //     {
    //         let operator = self.current_token.value.clone();
    //         self.advance();
    //         let right = self.parse_factor()?;
    //         left = Expression::BinaryOperation(BinaryOperation {
    //             left: Box::new(left),
    //             right: Box::new(right),
    //             operator,
    //             span: todo!(),
    //         });
    //     }

    //     Ok(left)
    // }

    /* This method wil handle addition and subtraction, which have the lowest precedence */
    // fn parse_expression(&mut self) -> Result<Expression, ParserError> {
    //     let mut term = self.parse_term()?;

    //     while self.current_token.token_type == TokenType::Operator
    //         && (self.current_token.value.as_str() == "+"
    //             || self.current_token.value.as_str() == "-")
    //     {
    //         let operator = self.current_token.value.clone();
    //         self.advance();
    //         let next_term = self.parse_term()?;
    //         term = Expression::BinaryOperation(BinaryOperation {
    //             left: Box::new(term),
    //             right: Box::new(next_term),
    //             operator,
    //             span: todo!(),
    //         });
    //     }

    //     Ok(term)
    // }
}

#[cfg(test)]
mod tests {
    use crate::parser::{
        expressions::{BinaryExpression, BinaryOperation, Literal, NumberLiteral},
        nodes::program::Program,
        ArithmeticOperator, Expression, Identifier, Node, Parser, Statement, VariableDeclaration,
        VariableDeclarator, VariableKind,
    };
    use pretty_assertions::assert_eq;
    use string_cache::Atom;

    fn num_literal(value: f64, start: usize, end: usize) -> Box<BinaryExpression> {
        Box::new(BinaryExpression::Literal(Literal::Number(NumberLiteral {
            node: Node::new(start, end),
            value,
        })))
    }

    #[test]
    fn empty_program() {
        let source_code = "";
        let mut parser = Parser::new(&source_code);
        let result = parser.parse();

        assert_eq!(
            result,
            Ok(Program {
                node: Node::new(0, 0),
                shebang: None,
                body: vec![],
            })
        );
    }

    #[test]
    fn variable_decl() {
        let source_code = "let a = 50.5";
        let mut parser = Parser::new(&source_code);
        let result = parser.parse();

        let expected = Program {
            node: Node::new(0, source_code.len()),
            shebang: None,
            body: vec![Statement::VariableDeclaration(Box::new(
                VariableDeclaration {
                    node: Node::new(0, source_code.len()),
                    declarations: vec![VariableDeclarator {
                        node: Node::new(4, source_code.len()),
                        id: Identifier::new(Atom::from("a"), 4, 5),
                        init: Some(Expression::Literal(Box::new(Literal::Number(
                            NumberLiteral {
                                node: Node::new(8, source_code.len()),
                                value: 50.5,
                            },
                        )))),
                    }],
                    kind: VariableKind::Let,
                },
            ))],
        };

        assert_eq!(result, Ok(expected));
    }
}
