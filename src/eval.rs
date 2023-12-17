use crate::{env::*, parse::*};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

fn eval_symbol(symbol: &String, env: &mut Rc<RefCell<Env>>) -> Result<Value, String> {
    let val = env.borrow().get(symbol);

    if val.is_none() {
        return Err(format!("Unbound symbol: {}", symbol));
    }

    Ok(val.unwrap().clone())
}

fn eval_arithmetic_op(list: &[Value], env: &mut Rc<RefCell<Env>>) -> Result<Value, String> {
    let head = &list[0];
    let tail = &list[1..];

    match head {
        Value::Symbol(s) => match s.as_str() {
            "+" => {
                let mut r: f64 = 0.0;
                for v in tail {
                    match eval_value(v, env)? {
                        Value::Number(n) => r += n,
                        _ => return Err(String::from("All arguments must be numbers")),
                    }
                }

                Ok(Value::Number(r))
            }

            "-" => {
                // return the negative if there's only 1 argument
                if tail.len() == 1 {
                    return match eval_value(&tail[0], env)? {
                        Value::Number(n) => Ok(Value::Number(-n)),
                        _ => return Err(String::from("All arguments must be numbers")),
                    };
                }

                let mut r: f64 = match eval_value(&tail[0], env)? {
                    Value::Number(n) => n,
                    _ => return Err(String::from("All arguments must be numbers")),
                };

                for v in &tail[1..] {
                    match eval_value(v, env)? {
                        Value::Number(n) => r -= n,
                        _ => return Err(String::from("All arguments must be numbers")),
                    }
                }

                Ok(Value::Number(r))
            }

            "*" => {
                let mut r: f64 = 1.0;
                for v in tail {
                    match eval_value(v, env)? {
                        Value::Number(n) => r *= n,
                        _ => return Err(String::from("All arguments must be numbers")),
                    }
                }

                Ok(Value::Number(r))
            }

            "/" => {
                let mut r: f64 = match eval_value(&tail[0], env)? {
                    Value::Number(n) => n,
                    _ => return Err(String::from("All arguments must be numbers")),
                };
                for v in &tail[1..] {
                    match eval_value(v, env)? {
                        Value::Number(n) => r /= n,
                        _ => return Err(String::from("All arguments must be numbers")),
                    }
                }

                Ok(Value::Number(r))
            }

            _ => unreachable!(),
        },

        _ => unreachable!(),
    }
}

fn eval_comparison_op(list: &[Value], env: &mut Rc<RefCell<Env>>) -> Result<Value, String> {
    let head = &list[0];
    let tail = &list[1..];

    match head {
        Value::Symbol(s) => match s.as_str() {
            ">" => {
                let mut r: bool = true;
                if tail.len() < 2 {
                    return Err(String::from("\">\" requires at least 2 arguments"));
                }

                for i in 1..tail.len() {
                    match (eval_value(&tail[i - 1], env)?, eval_value(&tail[i], env)?) {
                        (Value::Number(a), Value::Number(b)) => {
                            if !(a > b) {
                                r = false;
                                break;
                            }
                        }
                        _ => return Err(String::from("All arguments must be numbers")),
                    }
                }

                if r {
                    Ok(Value::T)
                } else {
                    Ok(Value::Nil)
                }
            }

            "<" => {
                let mut r: bool = true;
                if tail.len() < 2 {
                    return Err(String::from("\"<\" requires at least 2 arguments"));
                }

                for i in 1..tail.len() {
                    match (eval_value(&tail[i - 1], env)?, eval_value(&tail[i], env)?) {
                        (Value::Number(a), Value::Number(b)) => {
                            if !(a < b) {
                                r = false;
                                break;
                            }
                        }
                        _ => return Err(String::from("All arguments must be numbers")),
                    }
                }

                if r {
                    Ok(Value::T)
                } else {
                    Ok(Value::Nil)
                }
            }

            ">=" => {
                let mut r: bool = true;
                if tail.len() < 2 {
                    return Err(String::from("\">=\" requires at least 2 arguments"));
                }

                for i in 1..tail.len() {
                    match (eval_value(&tail[i - 1], env)?, eval_value(&tail[i], env)?) {
                        (Value::Number(a), Value::Number(b)) => {
                            if !(a >= b) {
                                r = false;
                                break;
                            }
                        }
                        _ => return Err(String::from("All arguments must be numbers")),
                    }
                }

                if r {
                    Ok(Value::T)
                } else {
                    Ok(Value::Nil)
                }
            }

            "<=" => {
                let mut r: bool = true;
                if tail.len() < 2 {
                    return Err(String::from("\"<=\" requires at least 2 arguments"));
                }

                for i in 1..tail.len() {
                    match (eval_value(&tail[i - 1], env)?, eval_value(&tail[i], env)?) {
                        (Value::Number(a), Value::Number(b)) => {
                            if !(a <= b) {
                                r = false;
                                break;
                            }
                        }
                        _ => return Err(String::from("All arguments must be numbers")),
                    }
                }

                if r {
                    Ok(Value::T)
                } else {
                    Ok(Value::Nil)
                }
            }

            "=" => {
                let mut r: bool = true;
                if tail.len() < 2 {
                    return Err(String::from("\"=\" requires at least 2 arguments"));
                }

                for i in 1..tail.len() {
                    match (eval_value(&tail[i - 1], env)?, eval_value(&tail[i], env)?) {
                        (Value::Number(a), Value::Number(b)) => {
                            if !(a == b) {
                                r = false;
                                break;
                            }
                        }
                        _ => return Err(String::from("All arguments must be numbers")),
                    }
                }

                if r {
                    Ok(Value::T)
                } else {
                    Ok(Value::Nil)
                }
            }

            "!=" => {
                let mut r: bool = true;
                if tail.len() < 2 {
                    return Err(String::from("\"!=\" requires at least 2 arguments"));
                }

                for i in 1..tail.len() {
                    match (eval_value(&tail[i - 1], env)?, eval_value(&tail[i], env)?) {
                        (Value::Number(a), Value::Number(b)) => {
                            if !(a != b) {
                                r = false;
                                break;
                            }
                        }
                        _ => return Err(String::from("All arguments must be numbers")),
                    }
                }

                if r {
                    Ok(Value::T)
                } else {
                    Ok(Value::Nil)
                }
            }

            _ => unreachable!(),
        },

        _ => unreachable!(),
    }
}

fn eval_if(list: &[Value], env: &mut Rc<RefCell<Env>>) -> Result<Value, String> {
    // if cond then else
    if list.len() != 3 && list.len() != 4 {
        return Err(format!("\"if\" requires 2 or 3 arguments"));
    }

    let cond_obj = eval_value(&list[1], env)?;
    let cond = match cond_obj {
        Value::T => true,
        Value::Nil => false,
        _ => return Err(format!("Condition must be a bool")),
    };

    if cond {
        return eval_value(&list[2], env);
    } else {
        if list.len() == 4 {
            return eval_value(&list[3], env);
        } else {
            return Err(format!("No else branch found"));
        }
    }
}

fn eval_def(list: &[Value], env: &mut Rc<RefCell<Env>>) -> Result<Value, String> {
    // def symbol value
    if list.len() != 3 {
        return Err(format!("\"def\" requires 2 arguments"));
    }

    let sym = match &list[1] {
        Value::Symbol(s) => s.clone(),
        _ => return Err(format!("First parameter to \"def\" must be a symbol")),
    };
    let val = eval_value(&list[2], env)?;
    env.borrow_mut().set(&sym, val.clone());

    // return the value that was defined
    Ok(val)
}

fn eval_fun_definition(list: &[Value], _env: &mut Rc<RefCell<Env>>) -> Result<Value, String> {
    let params = match &list[1] {
        Value::List(list) => {
            let mut params = Vec::new();
            for param in list {
                match param {
                    Value::Symbol(s) => params.push(s.clone()),
                    _ => return Err(format!("Invalid parameter name")),
                }
            }
            params
        }
        _ => {
            return Err(String::from(
                "First parameter to \"lambda\" must be a list of parameter names",
            ))
        }
    };

    let body = Rc::from(list[2].clone());

    Ok(Value::Lambda {
        params,
        body,
        is_macro: false,
    })
}

fn eval_macro_definition(list: &[Value], _env: &mut Rc<RefCell<Env>>) -> Result<Value, String> {
    let function = eval_fun_definition(list, _env);
    match function {
        Ok(Value::Lambda { params, body, .. }) => Ok(Value::Lambda {
            params,
            body,
            is_macro: true,
        }),

        _ => function,
    }
}

fn eval_macro_expand(list: &[Value], env: &mut Rc<RefCell<Env>>) -> Result<Value, String> {
    if list.len() != 2 {
        return Err(String::from("\"macroexpand\" requires 1 argument"));
    }

    let value = list[1].clone();

    match eval_value(&value, env)? {
        Value::List(list) => {
            if list.len() > 1 {
                match eval_value(&list[0], env)? {
                    Value::Lambda { params, body, .. } => {
                        // macroexpand is really the same as calling the macro as a function (instead of a macro)

                        let mut new_value: Vec<Value> = vec![];
                        new_value.push(Value::Lambda {
                            params,
                            body,
                            is_macro: false,
                        });

                        for item in &list[1..] {
                            new_value.push(item.clone());
                        }

                        eval_value(&Value::List(new_value), env)
                    }

                    _ => Ok(value),
                }
            } else {
                Ok(value)
            }
        }

        _ => Ok(value),
    }
}

fn eval_fun_call(list: &[Value], env: &mut Rc<RefCell<Env>>) -> Result<Value, String> {
    let first = eval_value(&list[0], env)?;

    let fun = match first {
        Value::Symbol(s) => {
            let lamdba = env.borrow_mut().get(&s);
            if lamdba.is_none() {
                return Err(format!("Unbound symbol: {}", s));
            }

            lamdba.unwrap()
        }
        Value::Lambda { .. } => first.clone(),

        _ => return Err(String::from("First parameter is not a function")),
    };

    match fun {
        Value::Lambda {
            params,
            body,
            is_macro,
        } => {
            let mut new_env = Rc::new(RefCell::new(Env::extend(env.clone())));
            for (i, param) in params.iter().enumerate() {
                let val = eval_value(&list[i + 1], env)?;
                new_env.borrow_mut().set(param, val);
            }

            let value = eval_value(body.as_ref(), &mut new_env)?;

            if is_macro {
                // if it's a macro, evaluate the code it returns (with the calling code's environment)
                return eval_value(&value, env);
            } else {
                // if it's a function, just return its value
                return Ok(value);
            }
        }
        _ => unreachable!(),
    }
}

fn eval_let(list: &[Value], env: &mut Rc<RefCell<Env>>) -> Result<Value, String> {
    let bindings = match &list[1] {
        Value::List(list) => {
            let mut bindings = HashMap::new();
            for binding in list {
                match binding {
                    Value::List(l) => {
                        if l.len() == 2 {
                            match &l[0] {
                                Value::Symbol(s) => {
                                    bindings.insert(s, eval_value(&l[1], env)?);
                                }
                                _ => {
                                    return Err(format!(
                                        "The first parameter in each binding must be a symbol"
                                    ))
                                }
                            }
                        } else {
                            return Err(format!("Bindings need to be of the form (name value)"));
                        }
                    }
                    _ => return Err(format!("Bindings need to be of the form (name value)")),
                }
            }

            bindings
        }
        _ => {
            return Err(String::from(
                "First parameter to \"lambda\" must be a list of parameter names",
            ))
        }
    };

    let bodies: Vec<Value> = list[2..].to_vec();

    let mut new_env = Rc::new(RefCell::new(Env::extend(env.clone())));
    for (key, value) in bindings.iter() {
        new_env.borrow_mut().set(key, value.clone());
    }

    let mut last_value: Value = Value::Nil;
    for value in bodies {
        last_value = eval_value(&value, &mut new_env)?;
    }
    Ok(last_value)
}

fn eval_quote(list: &[Value], _env: &mut Rc<RefCell<Env>>) -> Result<Value, String> {
    if list.len() != 2 {
        return Err(String::from("\"quote\" requires 1 argument"));
    }

    Ok(list[1].clone())
}

fn eval_quasiquote_value(value: &Value, env: &mut Rc<RefCell<Env>>) -> Result<Value, String> {
    match value {
        Value::List(list) => {
            if list.len() == 0 {
                return Ok(Value::List(vec![]));
            }

            if list.len() == 1 {
                return Ok(Value::List(vec![eval_quasiquote_value(&list[0], env)?]));
            }

            if list[0] == Value::Symbol(String::from("unquote")) {
                if list.len() == 2 {
                    return Ok(list[1].clone());
                } else {
                    return Err(String::from("\"unquote\" requires 1 argument"));
                }
            }

            if list[0] == Value::Symbol(String::from("splice-unquote")) {
                return Err(String::from(
                    "Can't use \"splice-unquote\" directly under a \"quasiquote\"",
                ));
            }

            let mut new_list: Vec<Value> = vec![];
            for item in list {
                match item {
                    Value::List(list_item) => {
                        let mut pushed = false;
                        if list_item.len() >= 2 {
                            if list_item[0] == Value::Symbol(String::from("unquote")) {
                                if list_item.len() == 2 {
                                    new_list.push(eval_value(&list_item[1], env)?);
                                } else {
                                    return Err(String::from("\"unquote\" requires 1 argument"));
                                }
                                pushed = true;
                            }

                            if list_item[0] == Value::Symbol(String::from("splice-unquote")) {
                                if list_item.len() == 2 {
                                    let value = eval_value(&list_item[1], env)?;
                                    // if it's a regular value, just push it (same as unquote), but if it's a list, push its items one by one
                                    match value {
                                        Value::List(splice_items) => {
                                            for item in splice_items {
                                                new_list.push(item);
                                            }
                                        }

                                        _ => new_list.push(value),
                                    }
                                } else {
                                    return Err(String::from(
                                        "\"splice-unquote\" requires 1 argument",
                                    ));
                                }
                                pushed = true;
                            }
                        }

                        if !pushed {
                            new_list.push(eval_quasiquote_value(item, env)?);
                        }
                    }

                    _ => new_list.push(item.clone()),
                }
            }

            Ok(Value::List(new_list))
        }

        _ => Ok(value.clone()),
    }
}

fn eval_quasiquote(list: &[Value], env: &mut Rc<RefCell<Env>>) -> Result<Value, String> {
    if list.len() != 2 {
        return Err(String::from("\"quasiquote\" requires 1 argument"));
    }

    let value = &list[1];
    eval_quasiquote_value(value, env)
}

fn eval_list(list: &[Value], env: &mut Rc<RefCell<Env>>) -> Result<Value, String> {
    // handle empty list
    if list.len() == 0 {
        return Ok(Value::Nil);
    }

    let head = &list[0];
    match head {
        Value::Symbol(s) => match s.as_str() {
            "+" | "-" | "*" | "/" => {
                return eval_arithmetic_op(&list, env);
            }
            "=" | "!=" | "<" | ">" | "<=" | ">=" => {
                return eval_comparison_op(&list, env);
            }
            "and" | "or" | "not" => {
                todo!()
            }
            "if" => eval_if(&list, env),
            "def" => eval_def(&list, env),
            "lambda" => eval_fun_definition(&list, env),
            "macro" => eval_macro_definition(&list, env),
            "macroexpand" => eval_macro_expand(&list, env),
            "let" => eval_let(&list, env),
            "quote" => eval_quote(&list, env),
            "quasiquote" => eval_quasiquote(&list, env),
            _ => eval_fun_call(&list, env),
        },

        _ => eval_fun_call(&list, env),
    }
}

pub fn eval_value(value: &Value, env: &mut Rc<RefCell<Env>>) -> Result<Value, String> {
    match value {
        Value::Nil => Ok(Value::Nil),
        Value::T => Ok(Value::T),
        Value::Number(n) => Ok(Value::Number(*n)),
        Value::String(s) => Ok(Value::String(s.clone())),
        Value::Symbol(s) => eval_symbol(s, env),
        Value::Lambda {
            params,
            body,
            is_macro,
        } => Ok(Value::Lambda {
            params: params.clone(),
            body: body.clone(),
            is_macro: is_macro.clone(),
        }),
        Value::List(l) => eval_list(l, env),
    }
}
