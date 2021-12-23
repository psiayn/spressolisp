use std::io::{self, Write};

use spressolisp::funcs::evaluate_expression;

fn main() {
    loop {
        let mut inp = String::new();
        print!("spresso> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut inp).unwrap();
        let input = inp.trim().to_string();
        if input == ".quit" {
            break;
        }
        match evaluate_expression(input) {
            Ok(res) => println!("{}", res),
            Err(err) => println!("{}", err),
        }
    }
    println!("goodbye!");
}
