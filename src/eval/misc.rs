use super::eval_value;
use crate::{env::*, parse::*};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub fn eval_symbol(symbol: &String, env: &mut Rc<RefCell<Env>>) -> Result<Value, String> {
    // if it's a keyword, just return it (it doesn't get evaluated by looking up its value)
    if symbol.starts_with(":") {
        return Ok(Value::Symbol(symbol.clone()));
    }

    let val = env.borrow().get(symbol);

    if val.is_none() {
        return Err(format!("Unbound symbol: {}", symbol));
    }

    Ok(val.unwrap().clone())
}

pub fn eval_if(list: &[Value], env: &mut Rc<RefCell<Env>>) -> Result<Value, String> {
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

pub fn eval_def(list: &[Value], env: &mut Rc<RefCell<Env>>) -> Result<Value, String> {
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

pub fn eval_let(list: &[Value], env: &mut Rc<RefCell<Env>>) -> Result<Value, String> {
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
