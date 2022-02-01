use spressolisp::{ast::Number, env::Env, eval::extract_num, evaluate_expression};

#[test]
fn test_loop_basic() {
    let mut env = Env::new();
    if let Ok(_) = evaluate_expression("(define x 100)".to_string(), &mut env) {
        if let Ok(_) = evaluate_expression("(loop (> x 50) (define x (- x 10)))".to_string(), &mut env) {
            if let Ok(res) = evaluate_expression("(print x)".to_string(), &mut env) {
                if let Ok(Number::Int(res)) = extract_num(res, &mut env) {
                    assert_eq!(res, 50);
                } else {
                    assert!(false, "Result is not an integer");
                }
            } else {
                assert!(false, "Error evaluating expression");
            }
        } else {
            assert!(false, "Error evaluating expression");
        }
    } else {
        assert!(false, "Error evaluating expression");
    }
}
