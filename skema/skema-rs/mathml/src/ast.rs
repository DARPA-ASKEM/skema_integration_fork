use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Operator {
    Add,
    Multiply,
    Equals,
    Divide,
    Subtract,
    Sqrt,
    // Catchall for operators we haven't explicitly defined as enum variants yet.
    Other(String),
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Multiply => write!(f, "*"),
            Operator::Equals => write!(f, "="),
            Operator::Divide => write!(f, "/"),
            Operator::Subtract => write!(f, "-"),
            Operator::Sqrt => write!(f, "√"),
            Operator::Other(op) => write!(f, "{}", op),
        }
    }
}

/// The MathExpression enum represents the corresponding element type in MathML 3
/// (https://www.w3.org/TR/MathML3/appendixa.html#parsing_MathExpression)
#[derive(Debug, PartialEq, Clone)]
pub enum MathExpression {
    Mi(String),
    Mo(Operator),
    Mn(String),
    Msqrt(Box<MathExpression>),
    Mrow(Vec<MathExpression>),
    Mfrac(Box<MathExpression>, Box<MathExpression>),
    Msup(Box<MathExpression>, Box<MathExpression>),
    Msub(Box<MathExpression>, Box<MathExpression>),
    Munder(Vec<MathExpression>),
    Mover(Vec<MathExpression>),
    Msubsup(Vec<MathExpression>),
    Mtext(String),
    Mstyle(Vec<MathExpression>),
    Mspace(String),
    MoLine(String),
}

/// The Math struct represents the corresponding element type in MathML 3
/// (https://www.w3.org/TR/MathML3/appendixa.html#parsing_math)
#[derive(Debug, PartialEq)]
pub struct Math {
    pub content: Vec<MathExpression>,
}

