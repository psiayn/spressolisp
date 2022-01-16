use std::fs::File;
use std::path::PathBuf;

use home::home_dir;
use spressolisp::{env::Env, evaluate_expression};

use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {

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
	File::create(history_path.clone())
	    .expect("creation of history file failed");
    }

    // create the env
    let mut env = Env::new();

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
		}
		match evaluate_expression(input, &mut env) {
		    Ok(res) => println!("{}", res),
		    Err(err) => println!("{}", err),
		};
	    },
	    Err(ReadlineError::Interrupted) => {
		break
	    },
	    Err(ReadlineError::Eof) => {
		break
	    },
	    Err(err) => {
		println!("REPL Error: {:?}", err);
		break
	    }
	}
    }
    println!("goodbye!");
    rl.save_history(&history_path).unwrap();
}
