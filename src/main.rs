use std::num::{ParseFloatError, ParseIntError};

use rustyline::error::ReadlineError;
use rustyline::Editor;

#[derive(Clone, Debug)]
enum Expr {
    Int(i64), 
    Float(f64), 
    Symbol(String), // could be a string or a variable
    List(Vec<Expr>), // list of these
}

impl ToString for Expr {
    fn to_string(&self) -> String {
        match self {
            Expr::Int(n) => format!("{}", n),
            Expr::Float(n) => format!("{}", n),
            Expr::Symbol(n) => format!("{}", n),
            Expr::List(n) => format!("{:#?}", n),
        }
    }
}

#[derive(Debug)]
enum Error {
    // SyntaxErr(u32, u32), // line number and character
    // UnbalancedParens(usize), // parenthesis are not balanced, contains number of parens expected
    Reason(String),
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match self {
            // Error::SyntaxErr(l, c) => format!("syntax error at line {}, col {}", l, c),
            // Error::UnbalancedParens(n) => format!("unbalanced parenthesis, expected {}", n),
            Error::Reason(reason) => reason.to_string(),
        }
    }
}

fn main() {
    let mut rl = Editor::<()>::new();
    // if rl.load_history("~/.spresso/history.txt").is_err() {
    //     println!("No previous history");
    // }
    loop {
        let readline = rl.readline("spresso> ");
        match readline {
            Ok(line) => {
                // rl.add_history_entry(line.as_str());
                println!("Line: {}", line);
                let tokens = tokenize(line);
                let (result, _) = parse(&tokens).unwrap();
                println!("Res: {}", result.to_string());
            },
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                break;
            },
            Err(ReadlineError::Eof) => {
                println!("^D");
                break;
            },
            Err(err) => {
                println!("ERROR: {:?}", err);
                break;
            }
        }
    }
    // rl.save_history("~/.spresso/history.txt").unwrap();
}

// tokenize the given string a return a list of tokens
fn tokenize(expr: String) -> Vec<String> {
    expr
        .replace("(", "( ")
        .replace(")", " )")
        .split_whitespace()
        .map(|x| x.to_string())
        .collect()
}

fn parse<'a>(tokens: &'a [String]) -> Result<(Expr, &'a [String]), Error> {
    let (token, rest) = tokens.split_first()
        .ok_or(
            Error::Reason("could not get token".to_string())
        )?;
    match &token[..] {
        "(" => read_seq(rest),
        // ")" => Err(Error::UnbalancedParens(1)),
        ")" => Err(Error::Reason("Unbalanced parameters".to_string())),
        _ => Ok((parse_atom(token), rest)),
    }
}

fn read_seq<'a>(tokens: &'a [String]) -> Result<(Expr, &'a [String]), Error> {
  let mut res: Vec<Expr> = vec![];
  let mut xs = tokens;
  loop {
    let (next_token, rest) = xs
      .split_first()
      .ok_or(Error::Reason("could not find closing `)`".to_string()))
      ?;
    if next_token == ")" {
      return Ok((Expr::List(res), rest)) // skip `)`, head to the token after
    }
    let (exp, new_xs) = parse(&xs)?;
    res.push(exp);
    xs = new_xs;
  }
}

fn parse_atom(token: &str) -> Expr {
    let potential_int: Result<i64, ParseIntError> = token.parse();
    match potential_int {
        Ok(i) => Expr::Int(i),
        Err(_) => {
            let potential_float: Result<f64, ParseFloatError> = token.parse();
            match potential_float {
                Ok(v) => Expr::Float(v),
                Err(_) => Expr::Symbol(token.to_string().clone())
            }
        }
    }
}