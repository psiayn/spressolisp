use crate::{
    ast::{Atom, Expr, ExprKind, Lambda},
    env::Env,
    errors::{RuntimeError, SpressoError},
    eval::execute_single,
    TokenGiver, TokenHoarder,
};

pub fn lambda(args: Vec<Expr>, env: &mut Env) -> Result<Expr, SpressoError> {
    if args.len() < 2 {
        return Err(SpressoError::from(RuntimeError::from(
            "A lambda definition must have a param list and a body (any number of lists)",
        ))
        .maybe_with_tokens(args.get_tokens()));
    }

    let fn_params = args[0].clone();
    let body = args[1..].to_vec();

    match fn_params.kind {
        ExprKind::Atom(Atom::Symbol(ref fn_param)) => Ok(ExprKind::Lambda(
            Lambda::new(vec![fn_param.clone()], body, env.get_current_scopes())
                .maybe_with_tokens(fn_params.get_tokens()),
        )
        .into()),
        ExprKind::List(ref fn_params) => {
            let params: Result<Vec<String>, SpressoError> = fn_params
                .clone()
                .into_iter()
                .map(|param| {
                    if let ExprKind::Atom(Atom::Symbol(param)) = param.kind {
                        Ok(param)
                    } else {
                        Err(SpressoError::from(RuntimeError::from(
                            "lambda parameters must be a symbol",
                        ))
                        .maybe_with_tokens(param.get_tokens()))
                    }
                })
                .collect();

            Ok(ExprKind::Lambda(
                Lambda::new(params?, body, env.get_current_scopes())
                    .maybe_with_tokens(fn_params.get_tokens()),
            )
            .into())
        }
        _ => Err(
            SpressoError::from(RuntimeError::from("lambda parameters must be a symbol"))
                .maybe_with_tokens(fn_params.get_tokens()),
        ),
    }
}

pub fn execute_lambda(
    lambda: Lambda,
    args: Vec<Expr>,
    env: &mut Env,
) -> Result<Expr, SpressoError> {
    let args: Result<Vec<Expr>, SpressoError> = args
        .into_iter()
        .map(|arg| execute_single(arg, env))
        .collect();
    let args = args?;

    if args.len() != lambda.params.len() {
        Err(SpressoError::from(RuntimeError::from(format!(
            "Expected {} arguments, got {}",
            lambda.params.len(),
            args.len()
        )))
        .maybe_with_tokens(args.get_tokens())
        .maybe_with_tokens(lambda.get_tokens()))
    } else {
        env.in_given_scopes_and_new_scope(lambda.scopes.clone(), |env| {
            args.into_iter().enumerate().for_each(|(i, arg)| {
                env.insert(lambda.params[i].as_str(), arg);
            });

            // execute body
            let results = lambda
                .body
                .clone()
                .into_iter()
                .map(|expr| execute_single(expr, env));

            let mut last_ok_result = None;
            for result in results {
                if let Err(err) = result {
                    // return the first error we encounter
                    // note that the next expr in body is not executed after this
                    return Err(err);
                } else {
                    // store the last Ok() result we find
                    last_ok_result = Some(result)
                }
            }

            // return the last Ok() result (will be last expr if everything succeeded), or
            // return a unit (when body is empty)
            // NOTE: this technically isn't possible because the parsing for [`lambda`] (see fn
            // above) is such that it needs a body to be specified. Perhaps there's a way to
            // enforce this at compile time.
            last_ok_result.unwrap_or_else(|| Ok(ExprKind::Atom(Atom::Unit).into()))
        })
    }
}
