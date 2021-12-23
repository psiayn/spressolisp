pub mod ast;
mod env;
mod eval;
pub mod funcs {
    use crate::eval::execute;
    use crate::env::standard_env;
    use crate::ast::{Atom, Expr, Number, RuntimeError};
    pub fn evaluate_expression(input: String) -> Result<Expr, RuntimeError> {
        let mut tokenized_input: Vec<String> = tokenize(input);
        let mut global_env = standard_env();
        match read_from_tokens(&mut tokenized_input) {
            Ok(ast) => {
                // println!("{}", ast);
                match ast {
                    Expr::List(mut exprs) => execute(&mut exprs, &mut global_env),
                    _ => Err(RuntimeError::from(format!(
                        "Hmm I can't execute something that is not a list: {}",
                        ast
                    ))),
                }
            }
            Err(e) => Err(e),
        }
    }

    fn tokenize(input: String) -> Vec<String> {
        let input: String = input.replace("(", " ( ").replace(")", " ) ");
        let res = input
            .split_whitespace()
            .map(|tok| tok.to_string())
            .collect();
        return res;
    }

    fn read_from_tokens(tokens: &mut Vec<String>) -> Result<Expr, RuntimeError> {
        if tokens.len() == 0 {
            return Err(RuntimeError::from("Unexpected EOF".to_string()));
        }
        let token = tokens.remove(0);
        match token.as_str() {
            "(" => {
                if tokens.len() == 0 {
                    return Err(RuntimeError::from("'(' not closed"));
                }
                let mut ast: Vec<Expr> = Vec::new();
                while tokens[0] != ")" {
                    let inner_ast = match read_from_tokens(tokens) {
                        Ok(res) => res,
                        Err(e) => return Err(e),
                    };
                    ast.push(inner_ast);
                    if tokens.len() == 0 {
                        return Err(RuntimeError::from("'(' not closed"));
                    }
                }
                tokens.remove(0);
                return Ok(Expr::List(ast));
            }
            ")" => {
                return Err(RuntimeError::from("Unexpected ')'"))
            }
            _ => Ok(Expr::Atom(atom(token))),
        }
    }

    fn atom(token: String) -> Atom {
        match token.parse::<i64>() {
            Ok(num) => return Atom::Number(Number::Int(num)),
            Err(_) => match token.parse::<f64>() {
                Ok(num) => return Atom::Number(Number::Float(num)),
                Err(_) => Atom::Symbol(token),
            },
        }
    }
}
