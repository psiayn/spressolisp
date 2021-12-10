use std::io::{self, Write};

enum Expr {
    Atom(Atom),
    List(List),
}

enum Atom {
    Symbol(String),
    Number(Number),
}

enum Number {
    Int(i64),
    Float(f64),
}

enum List {
    Symbol(Vec<String>),
    Number(Vec<Number>),
}

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
        let tokenized_input: Vec<String> = tokenize(input);
        println!("{:?}", tokenized_input);
    }
    println!("goodbye!");
}

fn tokenize(input: String) -> Vec<String> {
    let input: String = input.replace("(", " ( ").replace(")", " ) ");
    let res = input
        .split_whitespace()
        .map(|tok| tok.to_string())
        .collect();
    return res;
}
