use spressolisp::{ast::{Number, Expr, Atom}, env::Env, eval::extract_num, evaluate_expression};

#[test]
fn test_string_nospace() {
    let mut env = Env::new();
    if let Ok(_) = evaluate_expression("(define x \"helloworld\")".to_string(), &mut env) {
        if let Ok(res) = evaluate_expression("(x)".to_string(), &mut env) {
            if let Expr::Atom(Atom::String(s)) = res {
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
            if let Expr::Atom(Atom::String(s)) = res {
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