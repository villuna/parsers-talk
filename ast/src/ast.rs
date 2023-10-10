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
