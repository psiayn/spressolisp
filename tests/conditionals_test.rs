use spressolisp::{ast::{Atom, Expr, Number}, env::Env, eval::extract_num, evaluate_expression};

#[test]
fn test_relops() {
    let mut env = Env::new();
    if let Ok(Expr::Atom(Atom::Bool(res))) = evaluate_expression("(> 1 2)".to_string() , &mut env) {
        assert!(!res, "Expected false but got true");
    } else {
        assert!(false, "Error evaluating expression");
    }

    if let Ok(Expr::Atom(Atom::Bool(res))) = evaluate_expression("(< 1 2)".to_string() , &mut env) {
        assert!(res, "Expected true but got false");
    } else {
        assert!(false, "Error evaluating expression");
    }

    if let Ok(Expr::Atom(Atom::Bool(res))) = evaluate_expression("(>= 1 2)".to_string() , &mut env) {
        assert!(!res, "Expected false but got true");
    } else {
        assert!(false, "Error evaluating expression");
    }

    if let Ok(Expr::Atom(Atom::Bool(res))) = evaluate_expression("(<= 2 2)".to_string() , &mut env) {
        assert!(res, "Expected true but got false");
    } else {
        assert!(false, "Error evaluating expression");
    }

    if let Ok(Expr::Atom(Atom::Bool(res))) = evaluate_expression("(== 2 2)".to_string() , &mut env) {
        assert!(res, "Expected true but got false");
    } else {
        assert!(false, "Error evaluating expression");
    }

    if let Ok(Expr::Atom(Atom::Bool(res))) = evaluate_expression("(!= 1 2)".to_string() , &mut env) {
        assert!(res, "Expected true but got false");
    } else {
        assert!(false, "Error evaluating expression");
    }
}

#[test]
fn test_conditional_ops() {
    let mut env = Env::new();
    if let Ok(Expr::Atom(Atom::Bool(res))) = evaluate_expression("(not true)".to_string(), &mut env) {
        assert!(!res, "Expected false but got true");
    } else {
        assert!(false, "Error evaluating expression");
    }

    if let Ok(Expr::Atom(Atom::Bool(res))) = evaluate_expression("(and (> 1 2) (> 3 2))".to_string(), &mut env) {
        assert!(!res, "Expected false but got true");
    } else {
        assert!(false, "Error evaluating expression");
    }

    if let Ok(Expr::Atom(Atom::Bool(res))) = evaluate_expression("(or (> 1 2) (> 3 2))".to_string(), &mut env) {
        assert!(res, "Expected true but got false");
    } else {
        assert!(false, "Error evaluating expression");
    }
}

#[test]
fn test_conditional() {
    let mut env = Env::new();
    if let Ok(res) = evaluate_expression("(if true (print 10) (print 11))".to_string(), &mut env) {
        if let Ok(Number::Int(res)) = extract_num(res, &mut env) {
            assert_eq!(res, 10);
        } else {
            assert!(false, "Result is not an integer");
        }
    } else {
        assert!(false, "Error evaluating expression");
    }
}
