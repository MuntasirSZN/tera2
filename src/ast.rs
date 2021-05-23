use std::fmt;

use crate::lexer::Operator;

// TODO: have a Span trait/struct for location for error reporting?

/// Whether to remove the whitespace of a `{% %}` tag
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ws {
    /// `true` if the tag is `{%-`, `{{-`, `{#-`
    pub left: bool,
    /// `true` if the tag is `-%}`, `-}}`, `-#}`
    pub right: bool,
}

impl Default for Ws {
    fn default() -> Self {
        Ws {
            left: false,
            right: false,
        }
    }
}

// /// All math operators
// #[derive(Copy, Clone, Debug, PartialEq)]
// pub enum MathOperator {
//     Add,
//     Sub,
//     Mul,
//     Div,
//     Modulo,
// }
//
// impl fmt::Display for MathOperator {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(
//             f,
//             "{}",
//             match *self {
//                 MathOperator::Add => "+",
//                 MathOperator::Sub => "-",
//                 MathOperator::Mul => "*",
//                 MathOperator::Div => "/",
//                 MathOperator::Modulo => "%",
//             }
//         )
//     }
// }

// /// All logic operators
// #[derive(Copy, Clone, Debug, PartialEq)]
// pub enum LogicOperator {
//     LessThan,
//     GreaterThan,
//     LessThanOrEqual,
//     GreaterThanOrEqual,
//     Equal,
//     NotEqual,
//     And,
//     Or,
// }
//
// impl fmt::Display for LogicOperator {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(
//             f,
//             "{}",
//             match *self {
//                 LogicOperator::GreaterThan => ">",
//                 LogicOperator::GreaterThanOrEqual => ">=",
//                 LogicOperator::LessThan => "<",
//                 LogicOperator::LessThanOrEqual => "<=",
//                 LogicOperator::Equal => "==",
//                 LogicOperator::NotEqual => "!=",
//                 LogicOperator::And => "and",
//                 LogicOperator::Or => "or",
//             }
//         )
//     }
// }

/// An expression is the node found in variable block, kwargs and conditions.
#[derive(Clone, Debug, PartialEq)]
#[allow(missing_docs)]
pub enum Expression {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Ident(String),
    Array(Vec<Expression>),
    Test(String, Vec<Expression>),
    Expr(Operator, Vec<Expression>),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Expression::*;

        match self {
            String(i) => write!(f, "{}", i),
            Int(i) => write!(f, "{}", i),
            Float(i) => write!(f, "{}", i),
            Ident(i) => write!(f, "{}", i),
            Bool(i) => write!(f, "{}", i),
            Array(vals) => {
                write!(f, "[")?;
                for (i, s) in vals.iter().enumerate() {
                    if i == vals.len() - 1 {
                        write!(f, "{}", s)?
                    } else {
                        write!(f, "{}, ", s)?
                    }
                }
                write!(f, "]")
            }
            Test(name, rest) => {
                write!(f, "is {}", name)?;

                if !rest.is_empty() {
                    write!(f, "(",)?;
                    for s in rest {
                        write!(f, " {}", s)?
                    }
                    write!(f, ")",)?;
                }
                Ok(())
            }
            Expr(op, rest) => {
                write!(f, "({}", op)?;
                for s in rest {
                    write!(f, " {}", s)?
                }
                write!(f, ")")
            }
        }
    }
}
