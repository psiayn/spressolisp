use spressolisp::{
    ast::{Atom, ExprKind},
    env::Env,
    evaluate_expression,
};

#[test]
fn test_string_nospace() {
    let mut env = Env::new();
    if let Ok(_) = evaluate_expression("(define x \"helloworld\")".to_string(), &mut env) {
        if let Ok(res) = evaluate_expression("(x)".to_string(), &mut env) {
            if let ExprKind::Atom(Atom::String(s)) = res.kind {
                assert_eq!(s, "helloworld");
            } else {
                assert!(false, "Result is not an integer");
            }
        } else {
            assert!(false, "Error evaluating expression");
        }
    } else {
        assert!(false, "Error evaluating expression");
    }
}

#[test]
fn test_string_space() {
    let mut env = Env::new();
    if let Ok(_) = evaluate_expression("(define x \"hello world\")".to_string(), &mut env) {
        if let Ok(res) = evaluate_expression("(x)".to_string(), &mut env) {
            if let ExprKind::Atom(Atom::String(s)) = res.kind {
                assert_eq!(s, "hello world");
            } else {
                assert!(false, "Result is not an integer");
            }
        } else {
            assert!(false, "Error evaluating expression");
        }
    } else {
        assert!(false, "Error evaluating expression");
    }
}
