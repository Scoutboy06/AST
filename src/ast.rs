use std::rc::Rc;

pub struct Span {
    start: LineColumn,
    end: LineColumn,
}

pub struct LineColumn {
    line: usize,
    column: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ASTNode {
    Program(Rc<Program>),
    VariableDeclaration(Rc<VariableDeclaration>),
    BinaryExpression(Rc<BinaryExpression>),
    IfStatement,
    WhileStatement,
    ReturnStatement,
    Block,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    start: usize,
    end: usize,
    body: Vec<ASTNode>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableDeclaration {
    start: usize,
    end: usize,
    declarations: Vec<VariableDeclarator>,
    kind: VariableKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VariableKind {
    Let,
    Const,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableDeclarator {
    start: usize,
    end: usize,
    id: Identifier,
    init: Option<ASTNode>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier {
    start: usize,
    end: usize,
    name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Int(i64),
    Float(f64),
    BinaryOperation(BinaryExpression),
    Variable(Identifier),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryExpression {
    pub left: Rc<BinaryExpression>,
    pub right: Rc<BinaryExpression>,
    pub operator: String,
}

// impl BinaryExpression {
//     pub fn evaluate(&self) -> BinaryOperationResult {
//         let left = match self.left.as_ref() {
//             BinaryExpression::BinaryOperation(op) => op.evaluate(),
//             BinaryExpression::Float(float) => BinaryOperationResult::Float(*float),
//             BinaryExpression::Int(int) => BinaryOperationResult::Int(*int),
//         };

//         let right = match self.right.as_ref() {
//             BinaryExpression::BinaryOperation(op) => op.evaluate(),
//             BinaryExpression::Float(float) => BinaryOperationResult::Float(*float),
//             BinaryExpression::Int(int) => BinaryOperationResult::Int(*int),
//         };

//         let result = match self.operator.as_str() {
//             "+" => left.add(&right),
//             "-" => left.sub(&right),
//             "*" => left.mult(&right),
//             "/" => left.div(&right),
//             "**" => todo!(),
//             _ => unreachable!(),
//         };

//         result
//     }
// }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryOperationResult {
    Int(i64),
    Float(f64),
    // variable expressions, etc.
}

impl BinaryOperationResult {
    fn as_f64(&self) -> f64 {
        match *self {
            BinaryOperationResult::Int(int) => int as f64,
            BinaryOperationResult::Float(float) => float,
        }
    }

    fn add(&self, other: &BinaryOperationResult) -> BinaryOperationResult {
        match (self, other) {
            (BinaryOperationResult::Int(int_left), BinaryOperationResult::Int(int_right)) => {
                BinaryOperationResult::Int(*int_left + *int_right)
            }
            _ => {
                let left = self.as_f64();
                let right = other.as_f64();
                BinaryOperationResult::Float(left + right)
            }
        }
    }

    fn sub(&self, other: &BinaryOperationResult) -> BinaryOperationResult {
        match (self, other) {
            (BinaryOperationResult::Int(int_left), BinaryOperationResult::Int(int_right)) => {
                BinaryOperationResult::Int(*int_left - *int_right)
            }
            _ => {
                let left = self.as_f64();
                let right = other.as_f64();
                BinaryOperationResult::Float(left - right)
            }
        }
    }

    fn mult(&self, other: &BinaryOperationResult) -> BinaryOperationResult {
        match (self, other) {
            (BinaryOperationResult::Int(int_left), BinaryOperationResult::Int(int_right)) => {
                BinaryOperationResult::Int(*int_left * *int_right)
            }
            _ => {
                let left = self.as_f64();
                let right = other.as_f64();
                BinaryOperationResult::Float(left * right)
            }
        }
    }

    fn div(&self, other: &BinaryOperationResult) -> BinaryOperationResult {
        match (self, other) {
            (BinaryOperationResult::Int(int_left), BinaryOperationResult::Int(int_right)) => {
                BinaryOperationResult::Int(*int_left / *int_right)
            }
            _ => {
                let left = self.as_f64();
                let right = other.as_f64();
                BinaryOperationResult::Float(left / right)
            }
        }
    }
}
