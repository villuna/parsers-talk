//! interpreter.rs - naive aioi interpreter
//!
//! the interpreter takes in an abstract syntax tree and attempts to run it.
//! will panic if it runs into any sticky situations, like trying to use
//! values as the wrong type.

use std::collections::HashMap;

use crate::ast::{Ast, BinaryOperation, Expression, Statement};

#[derive(Copy, Clone, Debug, PartialEq)]
enum Value {
    Int(i32),
    Bool(bool),
}

fn eval(expr: &Expression, vars: &HashMap<String, Value>) -> Value {
    match expr {
        Expression::Variable(s) => {
            let Some(val) = vars.get(s) else {
                panic!("variable does not exist: {s}");
            };

            *val
        }

        Expression::Int(i) => Value::Int(*i),
        Expression::Bool(b) => Value::Bool(*b),

        Expression::BinaryOperation(op, v1, v2) => match op {
            BinaryOperation::GreaterThan => {
                let Value::Int(i1) = eval(v1, vars) else {
                    panic!("cannot coerce to int")
                };
                let Value::Int(i2) = eval(v2, vars) else {
                    panic!("cannot coerce to int")
                };

                Value::Bool(i1 > i2)
            }
            BinaryOperation::EqualTo => match eval(v1, vars) {
                Value::Int(i1) => {
                    let Value::Int(i2) = eval(v2, vars) else {
                        panic!("cannot coerce to int")
                    };
                    Value::Bool(i1 == i2)
                }
                Value::Bool(b1) => {
                    let Value::Bool(b2) = eval(v2, vars) else {
                        panic!("cannot coerce to bool")
                    };
                    Value::Bool(b1 == b2)
                }
            },
            BinaryOperation::Add => {
                let Value::Int(i1) = eval(v1, vars) else {
                    panic!("cannot coerce to int")
                };
                let Value::Int(i2) = eval(v2, vars) else {
                    panic!("cannot coerce to int")
                };

                Value::Int(i1 + i2)
            }
        },
    }
}

fn run_block(block: &[Statement], vars: &mut HashMap<String, Value>) {
    for statement in block {
        match statement {
            Statement::Declaration { id, rvalue } => {
                vars.insert(id.clone(), eval(rvalue, vars));
            }
            Statement::IfStatement { condition, block } => {
                if let Value::Bool(c) = eval(condition, vars) {
                    if c {
                        run_block(block, vars);
                    }
                } else {
                    panic!("condition was not boolean!")
                }
            }
            Statement::Print { value } => match eval(value, vars) {
                Value::Int(i) => println!("{i}"),
                Value::Bool(b) => println!("{b}"),
            },
        }
    }
}

/// A naive interpreter
pub fn run(ast: &Ast) {
    let mut vars = HashMap::new();
    run_block(&ast.statements, &mut vars);
}
