use std::env;
use std::fs::{read_to_string, File};
use std::path::PathBuf;

use home::home_dir;
use spressolisp::{env::Env, evaluate_expression};

use rustyline::error::ReadlineError;
use rustyline::Editor;

fn repl() {
    // readline REPL
    let mut rl = Editor::<()>::new();

    // creating path to history file
    let mut path = PathBuf::new();
    path.push(home_dir().unwrap());
    path.push(".spresso_history");
    let history_path = path;

    // check if history doesn't exist and create it
    if rl.load_history(&history_path.clone()).is_err() {
        println!("Welcome to spressolisp!");
        // hack to create file if it doesn't exist for now
        File::create(history_path.clone()).expect("creation of history file failed");
    }

    // create the env
    let mut env = Env::new();
    let mut input_num = 0;

    // start the eval loop
    loop {
        // except Ok, everything else is error handling and boilerplate
        // for the readline REPL
        let readline = rl.readline("spresso> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());

                // real execution start here
                let input = line.trim().to_string();
                if input == ".quit" {
                    break;
                } else if input == ".env" {
                    env.display();
                } else {
                    match evaluate_expression(format!("input[{}]", input_num), input, &mut env) {
                        Ok(res) => println!("{}", res),
                        Err(err) => println!("{}", err),
                    };
                }

                input_num += 1;
            }
            Err(ReadlineError::Interrupted) => break,
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("REPL Error: {:?}", err);
                break;
            }
        }
    }
    println!("goodbye!");
    rl.save_history(&history_path).unwrap();
}

fn execute_file(filepath: &str) {
    let mut env = Env::new();

    let contents = read_to_string(filepath).expect("Could not read file");

    if let Err(err) = evaluate_expression(filepath.to_owned(), contents, &mut env) {
        println!("{}", err);
    };
}

fn main() {
    if let Some(filepath) = env::args().nth(1) {
        execute_file(&filepath);
    } else {
        repl()
    }
}
