use super::eval_value;
use crate::{env::*, parse::*};
use std::{cell::RefCell, rc::Rc};

pub fn eval_quote(list: &[Value], _env: &mut Rc<RefCell<Env>>) -> Result<Value, String> {
    if list.len() != 2 {
        return Err(String::from("\"quote\" requires 1 argument"));
    }

    Ok(list[1].clone())
}

pub fn eval_quasiquote_value(value: &Value, env: &mut Rc<RefCell<Env>>) -> Result<Value, String> {
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

pub fn eval_quasiquote(list: &[Value], env: &mut Rc<RefCell<Env>>) -> Result<Value, String> {
    if list.len() != 2 {
        return Err(String::from("\"quasiquote\" requires 1 argument"));
    }

    let value = &list[1];
    eval_quasiquote_value(value, env)
}
