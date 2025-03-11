use crate::{
    ast::{Atom, Expr, ExprKind, Macro},
    env::Env,
    errors::{RuntimeError, SpressoError},
    TokenGiver, TokenHoarder,
};

/// Define a macro
/// # Usage
/// `(defmacro name (params) body)`
pub fn defmacro(args: Vec<Expr>, env: &mut Env) -> Result<Expr, SpressoError> {
    if args.len() < 3 {
        return Err(SpressoError::from(RuntimeError::from(
            "Macro definition needs a name, parameter list, and body",
        ))
        .maybe_with_tokens(args.get_tokens()));
    }

    // Get the macro name
    let name = if let ExprKind::Atom(Atom::Symbol(name)) = &args[0].kind {
        name.clone()
    } else {
        return Err(SpressoError::from(RuntimeError::from(
            "First argument to defmacro must be a symbol",
        ))
        .maybe_with_tokens(args[0].get_tokens()));
    };

    // Get the parameter list
    let params = if let ExprKind::List(param_list) = &args[1].kind {
        param_list
            .iter()
            .map(|param| {
                if let ExprKind::Atom(Atom::Symbol(name)) = &param.kind {
                    Ok(name.clone())
                } else {
                    Err(SpressoError::from(RuntimeError::from(
                        "Parameters must be symbols",
                    ))
                    .maybe_with_tokens(param.get_tokens()))
                }
            })
            .collect::<Result<Vec<String>, SpressoError>>()?
    } else {
        return Err(SpressoError::from(RuntimeError::from(
            "Second argument to defmacro must be a list of parameters",
        ))
        .maybe_with_tokens(args[1].get_tokens()));
    };

    // Get the body expressions
    let body = args[2..].to_vec();

    // Create the macro
    let macro_def = Macro {
        params,
        body,
        scopes: env.get_current_scopes(),
        param_tokens: args[1].get_tokens().unwrap_or_default(),
    };

    // Store the macro in the environment
    env.insert(&name, ExprKind::Macro(macro_def).into());

    Ok(ExprKind::Atom(Atom::Symbol(name)).into())
}

/// Expand a macro by replacing its parameters with the provided arguments
pub fn expand_macro(macro_def: &Macro, args: Vec<Expr>, env: &mut Env) -> Result<Expr, SpressoError> {
    if args.len() != macro_def.params.len() {
        return Err(SpressoError::from(RuntimeError::from(format!(
            "Macro expected {} arguments, got {}",
            macro_def.params.len(),
            args.len()
        )))
        .maybe_with_tokens(args.get_tokens()));
    }

    // Create a new scope for macro expansion
    env.in_given_scopes_and_new_scope(macro_def.scopes.clone(), |env| {
        // Bind arguments to parameters
        for (param, arg) in macro_def.params.iter().zip(args.iter()) {
            env.insert(param, arg.clone());
        }

        // Expand the macro body
        let mut result = ExprKind::Atom(Atom::Unit).into();
        for expr in macro_def.body.iter() {
            result = expand_expr(expr.clone(), env)?;
        }
        Ok(result)
    })
}

/// Recursively expand expressions, handling macro calls
fn expand_expr(expr: Expr, env: &mut Env) -> Result<Expr, SpressoError> {
    match &expr.kind {
        ExprKind::List(list) if !list.is_empty() => {
            // Check if the first element is a macro
            let first = &list[0];
            if let ExprKind::Atom(Atom::Symbol(name)) = &first.kind {
                if let Ok(macro_expr) = env.get_symbol(name) {
                    if let ExprKind::Macro(macro_def) = macro_expr.kind {
                        // It's a macro call, expand it
                        return expand_macro(&macro_def, list[1..].to_vec(), env);
                    }
                }
            }
            
            // Not a macro call, recursively expand each element
            let expanded: Result<Vec<Expr>, SpressoError> = list
                .iter()
                .map(|e| expand_expr(e.clone(), env))
                .collect();
            Ok(ExprKind::List(expanded?).into())
        }
        ExprKind::List(list) => Ok(ExprKind::List(list.clone()).into()),
        ExprKind::Atom(Atom::Symbol(name)) => {
            // Try to resolve the symbol in the current environment
            if let Ok(value) = env.get_symbol(name) {
                Ok(value)
            } else {
                Ok(expr)
            }
        }
        _ => Ok(expr),
    }
} 