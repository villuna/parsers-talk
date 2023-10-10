mod ast;
mod interpreter;
mod parser;
mod token;

fn main() {
    let code = std::fs::read_to_string("test.aioi").unwrap();
    
    // First we take the code and turn it into a linear stream of "tokens"
    let stream = token::token_stream(&code).expect("couldn't parse token stream");

    // Then we turn those tokens into an abstract syntax tree
    let ast = parser::parse_program(&stream).expect("couldn't parse abstract syntax tree");

    //println!("[aioi compiler] Built abstract syntax tree:\n{:#?}\n", ast);

    println!("[aioi compiler] Running the program - output:\n");

    // Final
    interpreter::run(&ast);

    println!("\n[aioi compiler] Program exited normally.");
}
