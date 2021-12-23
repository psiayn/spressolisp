use spressolisp::{
    ast::{Atom, Expr, Number, RuntimeError},
    funcs,
};

fn extract_number_from_expr(result: Result<Expr, RuntimeError>) 
    -> Result<i64, RuntimeError> {
    match result? {
        Expr::Atom(atom) => match atom {
            Atom::Number(num) => match num {
                Number::Int(res) => Ok(res),
                _ => Err(RuntimeError {
                    err: "Wrong Type".to_string(),
                }),
            },
            _ => Err(RuntimeError {
                err: "Wrong Type".to_string(),
            }),
        },
        _ => Err(RuntimeError {
            err: "Wrong Type".to_string(),
        }),
    }
}

#[test]
fn test_addition() {
    let inp = funcs::evaluate_expression("(+ 12 32)".to_string());
    let result = extract_number_from_expr(inp).unwrap();
    assert_eq!(result, 44);
}

#[test]
fn test_wrong_syntax() {
    let inp = funcs::evaluate_expression("(+ 12 32".to_string());
    let result = extract_number_from_expr(inp);
    let result = match result {
        Ok(_) => panic!("Invalid Expression successfully evaluate"),
        Err(err) => err,
    };
    let expected = RuntimeError{ err: "'(' not closed".to_string() };
    assert_eq!(result.to_string(), expected.to_string());
}
