//! ast.rs - Abstract Syntax Tree
//! 
//! When parsing a programming language, the data type we want to turn our
//! code into is something called an abstract syntax tree (AST). This is a
//! parse tree that ignores the specific syntax of the program and just stores
//! each node as an abstract representation of that part of the program. We
//! can then pass this tree into a code generator or interpreter, depending on
//! what we're writing (here I've just made a naive interpreter in interpreter.rs).
//! 
//! It's outside of the scope of this talk, but this is covered more in courses
//! like COMP4403 (compilers and interpreters), and no doubt extensively online.

// There are many more operations we could support but this is enough for a demo
#[derive(PartialEq, Eq, Debug)]
pub enum BinaryOperation {
    GreaterThan,
    EqualTo,
    Add,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Expression {
    Variable(String),
    Int(i32),
    Bool(bool),

    // What's in the box?
    //
    // Box is rust's smart pointer for a value that lives on the heap
    // This is necessary because recursive types dont have fixed sizes,
    // so in order to make a type recursive we need some kind of indirection like this.
    // In if statements we use a Vec, but here we use Boxes.
    BinaryOperation(BinaryOperation, Box<Expression>, Box<Expression>),
}

#[derive(PartialEq, Eq, Debug)]
pub enum Statement {
    Declaration {
        id: String,
        rvalue: Expression,
    },
    IfStatement {
        condition: Expression,
        block: Vec<Statement>,
    },
    Print {
        value: Expression,
    },
}

#[derive(PartialEq, Eq, Debug)]
pub struct Ast {
    pub statements: Vec<Statement>,
}
