use spressolisp::{ast::Number, env::Env, eval::extract_num, evaluate_expression};

#[test]
fn test_check_define_in_scope() {
    let mut env = Env::new();
    if let Ok(_) = evaluate_expression("(define x 100)".to_string(), &mut env) {
        if env.contains_key("x") {
            assert!(true, "Env contains x");
        } else {
            assert!(false, "Env does not contain x");
        }
    } else {
        assert!(false, "Error evaluating expression");
    }
}

#[test]
fn test_check_define_res() {
    let mut env = Env::new();
    if let Ok(res) = evaluate_expression("(define x 100)".to_string(), &mut env) {
        if let Ok(Number::Int(res)) = extract_num(res, &mut env) {
            assert_eq!(res, 100);               
        } else {
            assert!(false, "Result was not an integer");
        }
    } else {
        assert!(false, "Error evaluating expression");
    }
}

#[test]
fn test_print() {
    let mut env = Env::new();
    if let Ok(res) = evaluate_expression("(print 100)".to_string(), &mut env) {
        if let Ok(Number::Int(res)) = extract_num(res, &mut env) {
            assert_eq!(res, 100);
        } else {
            assert!(false, "Result was not an integer");
        }
    } else {
        assert!(false, "Error evaluating expression");
    }
}
