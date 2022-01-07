use spressolisp::{
    ast::Number,
    env::Env,
    errors::SyntaxError,
    eval::extract_num,
    evaluate_expression,
};

#[test]
fn test_addition() {
    let mut env = Env::new();
    if let Ok(res) = evaluate_expression("(+ 12 32)".to_string(), &mut env) {
        if let Ok(Number::Int(res)) = extract_num(res, &mut env) {
            assert_eq!(res, 44);
        } else {
            assert!(false, "Result was not an integer");
        }
    } else {
        assert!(false, "Error evaluating expression");
    }
}

#[test]
fn test_wrong_syntax() {
    let mut env = Env::new();
    let inp = evaluate_expression("(+ 12 32".to_string(), &mut env);

    if let Err(err) = inp {
        let expected = SyntaxError {
            err: "'(' not closed".to_string(),
        };
        assert_eq!(err.to_string(), expected.to_string());
    } else {
        assert!(false, "Invalid Expression successfully evaluated");
    }
}